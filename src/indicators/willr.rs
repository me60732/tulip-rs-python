use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::willr as rust_willr;

/// WILLR State wrapper for Python
#[pyclass]
pub struct WillrState {
    inner: rust_willr::IndicatorState,
}

#[pymethods]
impl WillrState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "WILLR State - internal state for Williams %R".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays [high, low, close]
    ///
    /// Returns:
    ///     List of output arrays [willr]
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_willr::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "WILLR requires {} input arrays, got {}",
                rust_willr::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for three inputs (high, low, close)
        let inputs_array: [&[f64]; rust_willr::INPUTS_WIDTH] = [
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
}

/// Calculate WILLR (Williams %R)
///
/// Williams %R is a momentum indicator that measures overbought and oversold levels.
/// It oscillates between -100 and 0, with readings above -20 considered overbought
/// and readings below -80 considered oversold.
///
/// Parameters:
/// - inputs: List of numpy arrays [high, low, close]
/// - options: List containing [period]
/// - optional_outputs: Optional list of booleans for additional outputs (none available)
///
/// Returns:
/// - Tuple of (outputs, state) where outputs is [willr_line]
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, WillrState)> {
    if options.len() != rust_willr::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "WILLR requires exactly 1 option: period",
        ));
    }

    if inputs.len() != rust_willr::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "WILLR requires {} input arrays, got {}",
            rust_willr::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Direct extraction for three inputs (high, low, close)
    let inputs_array: [&[f64]; rust_willr::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];
    let options_array: [f64; rust_willr::OPTIONS_WIDTH] = [options[0]];

    match rust_willr::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => Ok((outputs, WillrState { inner: state })),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Calculation error: {}",
            e
        ))),
    }
}

/// Get WILLR indicator information
#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_willr::INFO)
}

/// Get minimum data length required for WILLR calculation
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_willr::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "WILLR requires exactly 1 option: period",
        ));
    }
    Ok(rust_willr::min_data(&options))
}

/// Get minimum data length required for WILLR calculation with accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != rust_willr::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "WILLR requires exactly 1 option: period",
        ));
    }
    Ok(rust_willr::min_data_accuracy(&options, decimals))
}

/// Get output length for WILLR calculation
#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_willr::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "WILLR requires exactly 1 option: period",
        ));
    }
    Ok(rust_willr::output_length(data_len, &options))
}

/// Calculate Williams %R for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// Williams %R
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [high, low, close] arrays
/// - options: Vector with 1 option [period]
/// - optional_outputs: Optional list of booleans for selecting outputs
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of WILLR results for each asset
///   - states: Vector of WillrState objects for continuing calculations
///
/// Input Structure:
/// The inputs parameter should be structured as:
/// ```
/// inputs = [
///     [high, low, close] asset1,  # Asset 1
///     [high, low, close] asset2,  # Asset 2
///     # ... more assets
/// ]
/// ```
///
/// Example:
/// ```python
/// import numpy as np
/// import tulip_rs as ti
///
/// inputs = [[high1, low1, close1], [high2, low2, close2], [high3, low3, close3], [high4, low4, close4]]
/// options = [14.0]
///
/// # Calculate WILLR for all assets using SIMD
/// outputs, states = ti.indicators.willr.simd_by_assets(inputs, options, None)
///
/// # outputs[0] contains WILLR values for asset 1
/// # outputs[1] contains WILLR values for asset 2
/// # outputs[2] contains WILLR values for asset 3
/// # outputs[3] contains WILLR values for asset 4
/// # states[0] contains the state for asset 1 (for continuation)
/// # states[1] contains the state for asset 2 (for continuation)
/// # states[2] contains the state for asset 3 (for continuation)
/// # states[3] contains the state for asset 4 (for continuation)
/// ```
///
/// Note: This function only supports SIMD lane counts (2, 4, 8, or 16 assets).
/// For other numbers of assets, use the regular indicator function for each asset individually.
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_assets(
    inputs: Vec<Vec<PyReadonlyArray1<f64>>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<WillrState>)> {
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
        if asset_inputs.len() != rust_willr::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_willr::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_willr::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_willr::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_willr::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; rust_willr::INPUTS_WIDTH] = [
            asset_inputs[0].as_slice()?, // high
            asset_inputs[1].as_slice()?, // low
            asset_inputs[2].as_slice()?, // close
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_willr::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: [f64; rust_willr::OPTIONS_WIDTH] = [options[0]];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_willr::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_willr::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_willr::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_willr::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_willr::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_willr::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_willr::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_willr::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let willr_states: Vec<WillrState> = states
                .into_iter()
                .map(|state| WillrState { inner: state })
                .collect();
            Ok((results, willr_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}


#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_options(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<Vec<f64>>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<WillrState>)> {
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

    if inputs.len() != rust_willr::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_willr::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != rust_willr::OPTIONS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                rust_willr::OPTIONS_WIDTH,
                opt.len()
            )));
        }
    }

    let input_arrays: [&[f64]; rust_willr::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
            inputs[1].as_slice()?,
            inputs[2].as_slice()?,
    ];

    let mut option_arrays: Vec<[f64; rust_willr::OPTIONS_WIDTH]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0]]);
    }

    let option_refs: Vec<&[f64; rust_willr::OPTIONS_WIDTH]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; rust_willr::OPTIONS_WIDTH]; 2] =
                option_refs.as_slice().try_into().unwrap();
            rust_willr::by_options::indicator::<2>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let opt_array: &[&[f64; rust_willr::OPTIONS_WIDTH]; 4] =
                option_refs.as_slice().try_into().unwrap();
            rust_willr::by_options::indicator::<4>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let opt_array: &[&[f64; rust_willr::OPTIONS_WIDTH]; 8] =
                option_refs.as_slice().try_into().unwrap();
            rust_willr::by_options::indicator::<8>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let opt_array: &[&[f64; rust_willr::OPTIONS_WIDTH]; 16] =
                option_refs.as_slice().try_into().unwrap();
            rust_willr::by_options::indicator::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let willr_states: Vec<WillrState> = states
                .into_iter()
                .map(|state| WillrState { inner: state })
                .collect();
            Ok((results, willr_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the WILLR indicator module with Python
///
/// This function creates a Python submodule for the WILLR indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_willr_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "willr")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<WillrState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}
