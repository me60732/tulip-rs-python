use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::macd as rust_macd;

/// MACD State wrapper for Python
#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct MacdState {
    inner: rust_macd::IndicatorState,
}

#[pymethods]
impl MacdState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "MACD State - internal state for Moving Average Convergence Divergence".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for MACD: just one array of real values)
    ///
    /// Returns:
    ///     List of output arrays (for MACD: macd_line, signal_line, histogram)
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        py: Python<'_>,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Py<PyArray1<f64>>>> {
        if inputs.len() != rust_macd::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "MACD requires {} input arrays, got {}",
                rust_macd::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (real)
        let inputs_array: [&[f64]; rust_macd::INPUTS_WIDTH] = [inputs[0].as_slice()?];

        match TIndicatorState::batch_indicator(
            &mut self.inner,
            &inputs_array,
            optional_outputs.as_deref(),
        ) {
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
        "MacdState(internal)".to_string()
    }
}

/// Moving Average Convergence Divergence - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 3], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for MACD: just one array of real values)
///     options: Array of options (for MACD: [short_period, long_period, signal_period])
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (MACD has 3 outputs: [macd_line, signal_line, histogram])
///     - state: MacdState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> real = np.array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], dtype=np.float64)
///     >>> inputs = [real]  # MACD takes 1 input array
///     >>> options = [12.0, 26.0, 9.0]  # short_period=12, long_period=26, signal_period=9
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(len(outputs))  # 3 outputs: macd_line, signal_line, histogram
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Py<PyArray1<f64>>>, MacdState)> {
    // Validate inputs count
    if inputs.len() != rust_macd::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "MACD requires {} input arrays, got {}",
            rust_macd::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != rust_macd::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_macd::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Validate periods
    if options[0] < 1.0 || options[1] <= options[0] || options[2] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Invalid periods: short_period >= 1, long_period > short_period, signal_period >= 1",
        ));
    }

    // Direct extraction for single input (MACD only takes 1 input)
    let inputs_array: [&[f64]; rust_macd::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    // Convert options to fixed-size array
    let options_array: [f64; rust_macd::OPTIONS_WIDTH] = [options[0], options[1], options[2]];

    match rust_macd::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = MacdState { inner: state };
            Ok((crate::utils::vecs_to_pyarrays(py, outputs), py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "MACD calculation error: {}",
            e
        ))),
    }
}

/// Get MACD info
#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_macd::INFO)
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_macd::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_macd::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_macd::min_data(&options))
}



/// Calculate MACD for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// Parameters:
/// - inputs: Vector of asset inputs for MACD calculation
/// - options: Vector of options for MACD calculation
/// - optional_outputs: Optional list of booleans for additional outputs
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of MACD results for each asset
///   - states: Vector of MacdState objects for continuing calculations
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
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<MacdState>)> {
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
        if asset_inputs.len() != rust_macd::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_macd::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_macd::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_macd::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_macd::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: Result<[&[f64]; rust_macd::INPUTS_WIDTH], _> = asset_inputs
            .iter()
            .map(|arr| arr.as_slice())
            .collect::<Result<Vec<_>, _>>()?
            .try_into();

        match input_array {
            Ok(arr) => asset_input_arrays.push(arr),
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Failed to convert input arrays",
                ))
            }
        }
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_macd::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: Result<[f64; rust_macd::OPTIONS_WIDTH], _> = options.try_into();
    let options_array = options_array.map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to convert options to array of length {}",
            rust_macd::OPTIONS_WIDTH
        ))
    })?;

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_macd::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_macd::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_macd::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_macd::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_macd::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_macd::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_macd::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_macd::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let indicator_states: Vec<MacdState> = states
                .into_iter()
                .map(|state| MacdState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                indicator_states,
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}

// Auto-register functions for MACD

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_options(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<Vec<f64>>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<MacdState>)> {
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

    if inputs.len() != rust_macd::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_macd::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != rust_macd::OPTIONS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                rust_macd::OPTIONS_WIDTH,
                opt.len()
            )));
        }
    }

    let input_arrays: [&[f64]; rust_macd::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    let mut option_arrays: Vec<[f64; rust_macd::OPTIONS_WIDTH]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0], opt[1], opt[2]]);
    }

    let option_refs: Vec<&[f64; rust_macd::OPTIONS_WIDTH]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; rust_macd::OPTIONS_WIDTH]; 2] =
                option_refs.as_slice().try_into().unwrap();
            rust_macd::by_options::indicator::<2>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let opt_array: &[&[f64; rust_macd::OPTIONS_WIDTH]; 4] =
                option_refs.as_slice().try_into().unwrap();
            rust_macd::by_options::indicator::<4>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let opt_array: &[&[f64; rust_macd::OPTIONS_WIDTH]; 8] =
                option_refs.as_slice().try_into().unwrap();
            rust_macd::by_options::indicator::<8>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let opt_array: &[&[f64; rust_macd::OPTIONS_WIDTH]; 16] =
                option_refs.as_slice().try_into().unwrap();
            rust_macd::by_options::indicator::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let macd_states: Vec<MacdState> = states
                .into_iter()
                .map(|state| MacdState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                macd_states,
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

pub fn register_macd_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "macd")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<MacdState>()?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}
