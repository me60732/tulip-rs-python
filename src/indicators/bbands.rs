use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::bbands::{
    BBands, Indicator, IndicatorByOptions, IndicatorState, INPUTS, OPTIONS,
};

/// Bollinger Bands State wrapper for Python
#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct BbandsState {
    inner: IndicatorState,
}

#[pymethods]
impl BbandsState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "Bollinger Bands State - internal state for Bollinger Bands".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for BBANDS: just one array of real values)
    ///
    /// Returns:
    ///     List of output arrays (for BBANDS: [lower_band, middle_band, upper_band])
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        py: Python<'_>,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Py<PyArray1<f64>>>> {
        if inputs.len() != INPUTS {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "BBANDS requires {} input arrays, got {}",
                INPUTS,
                inputs.len()
            )));
        }

        // Direct extraction for single input (BBANDS only takes 1 input)
        let inputs_array: [&[f64]; INPUTS] = [inputs[0].as_slice()?];

        match self
            .inner
            .batch_indicator(&inputs_array, optional_outputs.as_deref())
        {
            Ok(outputs) => Ok(crate::utils::vecs_to_pyarrays(py, outputs)),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Calculation error: {}",
                e
            ))),
        }
    }

    /// Implement Python's pickle protocol - returns state as Python dict/primitives
    fn __getstate__(&self) -> PyResult<HashMap<String, String>> {
        let serialized = serde_json::to_string(&self.inner).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Serialization error: {}", e))
        })?;
        let mut state = HashMap::new();
        state.insert("inner".to_string(), serialized);
        Ok(state)
    }

    /// Implement Python's pickle protocol - restores state from Python dict/primitives
    fn __setstate__(&mut self, state: HashMap<String, String>) -> PyResult<()> {
        if let Some(inner_str) = state.get("inner") {
            self.inner = serde_json::from_str(inner_str).map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Deserialization error: {}", e))
            })?;
            Ok(())
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Missing 'inner' key in state",
            ))
        }
    }

    fn __repr__(&self) -> String {
        "BbandsState(internal)".to_string()
    }
}

/// Bollinger Bands - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 2], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for BBANDS: just one array of real values)
///     options: Array of options (for BBANDS: [period, std_dev])
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (BBANDS has 3 outputs: [lower_band, middle_band, upper_band])
///     - state: BbandsState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> real = np.array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], dtype=np.float64)
///     >>> inputs = [real]  # BBANDS takes 1 input array
///     >>> options = [20.0, 2.0]  # period=20, std_dev=2
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(len(outputs))  # 3 outputs: lower_band, middle_band, upper_band
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Py<PyArray1<f64>>>, BbandsState)> {
    // Validate inputs count
    if inputs.len() != INPUTS {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "BBANDS requires {} input arrays, got {}",
            INPUTS,
            inputs.len()
        )));
    }

    if options.len() != OPTIONS {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            OPTIONS,
            options.len()
        )));
    }

    // Validate options
    if options[0] < 1.0 || options[1] <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Invalid options: period >= 1, std_dev > 0",
        ));
    }

    // Direct extraction for single input (BBANDS only takes 1 input)
    let inputs_array: [&[f64]; INPUTS] = [inputs[0].as_slice()?];

    // Convert options to fixed-size array
    let options_array: [f64; OPTIONS] = [options[0], options[1]];

    match BBands::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = BbandsState { inner: state };
            Ok((crate::utils::vecs_to_pyarrays(py, outputs), py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "BBANDS calculation error: {}",
            e
        ))),
    }
}

/// Get BBANDS info
#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, BBands::INFO)
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != OPTIONS {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            OPTIONS,
            options.len()
        )));
    }
    let options_array: [f64; OPTIONS] = [options[0], options[1]];
    Ok(BBands::min_data(&options_array))
}

/// Calculate BBANDS for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// Parameters:
/// - inputs: Vector of asset inputs for BBANDS calculation
/// - options: Vector of options for BBANDS calculation
/// - optional_outputs: Optional list of booleans for additional outputs
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of BBANDS results for each asset
///   - states: Vector of BbandsState objects for continuing calculations
///
/// Note: This function only supports SIMD lane counts (2, 4, 8, or 16 assets).
/// For other numbers of assets, use the regular indicator function for each asset individually.
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_assets(
    py: Python<'_>,
    inputs: Vec<Vec<PyReadonlyArray1<f64>>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<BbandsState>)> {
    if inputs.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "No assets provided",
        ));
    }

    let num_assets = inputs.len();

    // Validate SIMD lane count - only support powers of 2
    if !matches!(num_assets, 2 | 4 | 8 | 16) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "SIMD by assets only supports 2, 4, 8, or 16 assets. Got {}",
            num_assets
        )));
    }

    // Validate that each asset has the correct number of inputs
    for (asset_idx, asset_inputs) in inputs.iter().enumerate() {
        if asset_inputs.len() != INPUTS {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                INPUTS,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != OPTIONS {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            OPTIONS,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; INPUTS]> = Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; INPUTS] = [asset_inputs[0].as_slice()?];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; INPUTS]> = asset_input_arrays.iter().collect();

    let options_array: [f64; OPTIONS] = [options[0], options[1]];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; INPUTS]; 2] = input_refs.as_slice().try_into().unwrap();
            BBands::indicator_by_assets::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; INPUTS]; 4] = input_refs.as_slice().try_into().unwrap();
            BBands::indicator_by_assets::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; INPUTS]; 8] = input_refs.as_slice().try_into().unwrap();
            BBands::indicator_by_assets::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; INPUTS]; 16] = input_refs.as_slice().try_into().unwrap();
            BBands::indicator_by_assets::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let bbands_states: Vec<BbandsState> = states
                .into_iter()
                .map(|state| BbandsState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                bbands_states,
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}

// Auto-register functions for BBANDS

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_options(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<Vec<f64>>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<BbandsState>)> {
    if options.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "No options provided",
        ));
    }

    let num_options = options.len();

    // Validate SIMD lane count - only support powers of 2
    if !matches!(num_options, 2 | 4 | 8 | 16) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "SIMD by options only supports 2, 4, 8, or 16 options. Got {}",
            num_options
        )));
    }

    if inputs.len() != INPUTS {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            INPUTS,
            inputs.len()
        )));
    }

    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != OPTIONS {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                OPTIONS,
                opt.len()
            )));
        }
    }

    let input_arrays: [&[f64]; INPUTS] = [inputs[0].as_slice()?];

    let mut option_arrays: Vec<[f64; OPTIONS]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0], opt[1]]);
    }

    let option_refs: Vec<&[f64; OPTIONS]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; OPTIONS]; 2] = option_refs.as_slice().try_into().unwrap();
            BBands::indicator_by_options::<2>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        4 => {
            let opt_array: &[&[f64; OPTIONS]; 4] = option_refs.as_slice().try_into().unwrap();
            BBands::indicator_by_options::<4>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        8 => {
            let opt_array: &[&[f64; OPTIONS]; 8] = option_refs.as_slice().try_into().unwrap();
            BBands::indicator_by_options::<8>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        16 => {
            let opt_array: &[&[f64; OPTIONS]; 16] = option_refs.as_slice().try_into().unwrap();
            BBands::indicator_by_options::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let bbands_states: Vec<BbandsState> = states
                .into_iter()
                .map(|state| BbandsState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                bbands_states,
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

pub fn register_bbands_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "bbands")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;

    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<BbandsState>()?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}
