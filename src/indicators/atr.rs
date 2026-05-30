use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::atr as rust_atr;

/// ATR State wrapper for Python
#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct AtrState {
    inner: rust_atr::IndicatorState,
}

#[pymethods]
impl AtrState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "ATR State - internal state for Average True Range".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for ATR: [high, low, close] arrays)
    ///
    /// Returns:
    ///     List of output arrays (for ATR: [atr] or [atr, tr] if optional outputs enabled)
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_atr::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "ATR requires {} input arrays, got {}",
                rust_atr::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for three inputs (high, low, close)
        let inputs_array: [&[f64]; rust_atr::INPUTS_WIDTH] = [
            inputs[0].as_slice()?,
            inputs[1].as_slice()?,
            inputs[2].as_slice()?,
        ];

        match TIndicatorState::batch_indicator(
            &mut self.inner,
            &inputs_array,
            optional_outputs.as_deref(),
        ) {
            Ok(outputs) => Ok(outputs),
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
        "AtrState(internal)".to_string()
    }
}

/// Average True Range - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 1], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for ATR: [high, low, close] arrays)
///     options: Array of options (for ATR: just the period)
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (ATR has 1-2 outputs: [atr] or [atr, tr])
///     - state: AtrState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> high = np.array([82, 83, 84, 85, 86], dtype=np.float64)
///     >>> low = np.array([80, 81, 82, 83, 84], dtype=np.float64)
///     >>> close = np.array([81, 82, 83, 84, 85], dtype=np.float64)
///     >>> inputs = [high, low, close]  # ATR takes 3 input arrays
///     >>> options = [14.0]  # period = 14
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(outputs[0])  # ATR values
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, AtrState)> {
    // Validate inputs count
    if inputs.len() != rust_atr::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "ATR requires {} input arrays, got {}",
            rust_atr::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != rust_atr::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_atr::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Validate period
    if options[0] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Period must be >= 1",
        ));
    }

    // Direct extraction for three inputs (ATR takes high, low, close)
    let inputs_array: [&[f64]; rust_atr::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];

    // Convert options to fixed-size array
    let options_array: [f64; rust_atr::OPTIONS_WIDTH] = [options[0]];

    match rust_atr::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = AtrState { inner: state };
            Ok((outputs, py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "ATR calculation error: {}",
            e
        ))),
    }
}

/// Get ATR info
#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_atr::INFO)
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_atr::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_atr::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_atr::min_data(&options))
}

/// Get expected output length
#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_atr::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_atr::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_atr::output_length(data_length, &options))
}

/// Get minimum data required for accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != rust_atr::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_atr::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_atr::min_data_accuracy(&options, decimals))
}

/// Calculate ATR for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// Parameters:
/// - inputs: Vector of asset inputs for ATR calculation
/// - options: Vector of options for ATR calculation
/// - optional_outputs: Optional list of booleans for additional outputs
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of ATR results for each asset
///   - states: Vector of AtrState objects for continuing calculations
///
/// Note: This function only supports SIMD lane counts (2, 4, 8, or 16 assets).
/// For other numbers of assets, use the regular indicator function for each asset individually.
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_assets(
    inputs: Vec<Vec<PyReadonlyArray1<f64>>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<AtrState>)> {
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
        if asset_inputs.len() != rust_atr::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_atr::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_atr::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_atr::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_atr::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: Result<[&[f64]; rust_atr::INPUTS_WIDTH], _> = asset_inputs
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
    let input_refs: Vec<&[&[f64]; rust_atr::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: Result<[f64; rust_atr::OPTIONS_WIDTH], _> = options.try_into();
    let options_array = options_array.map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to convert options to array of length {}",
            rust_atr::OPTIONS_WIDTH
        ))
    })?;

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_atr::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_atr::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_atr::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_atr::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_atr::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_atr::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_atr::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_atr::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let atr_states: Vec<AtrState> = states
                .into_iter()
                .map(|state| AtrState { inner: state })
                .collect();
            Ok((results, atr_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}

// Auto-register functions for ATR

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_options(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<Vec<f64>>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<AtrState>)> {
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

    if inputs.len() != rust_atr::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_atr::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != rust_atr::OPTIONS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                rust_atr::OPTIONS_WIDTH,
                opt.len()
            )));
        }
    }

    let input_arrays: [&[f64]; rust_atr::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
            inputs[1].as_slice()?,
            inputs[2].as_slice()?,
    ];

    let mut option_arrays: Vec<[f64; rust_atr::OPTIONS_WIDTH]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0]]);
    }

    let option_refs: Vec<&[f64; rust_atr::OPTIONS_WIDTH]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; rust_atr::OPTIONS_WIDTH]; 2] =
                option_refs.as_slice().try_into().unwrap();
            rust_atr::by_options::indicator::<2>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let opt_array: &[&[f64; rust_atr::OPTIONS_WIDTH]; 4] =
                option_refs.as_slice().try_into().unwrap();
            rust_atr::by_options::indicator::<4>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let opt_array: &[&[f64; rust_atr::OPTIONS_WIDTH]; 8] =
                option_refs.as_slice().try_into().unwrap();
            rust_atr::by_options::indicator::<8>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let opt_array: &[&[f64; rust_atr::OPTIONS_WIDTH]; 16] =
                option_refs.as_slice().try_into().unwrap();
            rust_atr::by_options::indicator::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let atr_states: Vec<AtrState> = states
                .into_iter()
                .map(|state| AtrState { inner: state })
                .collect();
            Ok((results, atr_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

pub fn register_atr_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "atr")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<AtrState>()?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}