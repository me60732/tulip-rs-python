use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::trix as rust_trix;

/// TRIX State wrapper for Python
#[pyclass]
pub struct TrixState {
    inner: rust_trix::IndicatorState,
}

#[pymethods]
impl TrixState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "TRIX State - internal state for Triple Exponential Oscillator".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays [real]
    ///
    /// Returns:
    ///     List of output arrays [trix] + optional outputs [tema, dema, ema]
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_trix::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "TRIX requires {} input arrays, got {}",
                rust_trix::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (real)
        let inputs_array: [&[f64]; rust_trix::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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

/// Calculate TRIX (Triple Exponential Oscillator)
///
/// TRIX is a momentum oscillator that displays the percent rate of change
/// of a triple exponentially smoothed moving average.
///
/// Parameters:
/// - inputs: List of numpy arrays [real]
/// - options: List containing [period]
/// - optional_outputs: Optional list of booleans for additional outputs [tema, dema, ema]
///
/// Returns:
/// - Tuple of (outputs, state) where outputs is [trix_line] + optional outputs
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, TrixState)> {
    if options.len() != rust_trix::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_trix::OPTIONS_WIDTH,
            options.len()
        )));
    }

    if inputs.len() != rust_trix::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "TRIX requires {} input arrays, got {}",
            rust_trix::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Direct extraction for single input (real)
    let inputs_array: [&[f64]; rust_trix::INPUTS_WIDTH] = [inputs[0].as_slice()?];
    let options_array: [f64; rust_trix::OPTIONS_WIDTH] = [options[0]];

    match rust_trix::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => Ok((outputs, TrixState { inner: state })),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Calculation error: {}",
            e
        ))),
    }
}

/// Get TRIX indicator information
#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_trix::INFO)
}

/// Calculate Triple Exponential Oscillator for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// TRIX is a momentum oscillator that displays the percent rate of change
/// of a triple exponentially smoothed moving average.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [real] arrays
/// - options: Vector with 1 option [period]
/// - optional_outputs: Optional list of booleans for selecting outputs
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of TRIX results for each asset
///   - states: Vector of TrixState objects for continuing calculations
///
/// Input Structure:
/// The inputs parameter should be structured as:
/// ```
/// inputs = [
///     [real_asset1],  # Asset 1
///     [real_asset2],  # Asset 2
///     # ... more assets
/// ]
/// ```
///
/// Example:
/// ```python
/// import numpy as np
/// import tulip_rs as ti
///
/// # Data for 4 assets, 30 periods each (SIMD requires 2, 4, 8, or 16 assets)
/// real1 = np.array([81.59, 81.06, 82.87, 83.00, 83.61, 83.15, 82.84, 83.99, 84.55, 84.36, 85.53, 86.54, 86.89, 87.77, 87.29, 88.15, 89.44, 89.18, 88.67, 87.96, 89.43, 90.17, 90.78, 91.04, 90.58, 91.32, 90.13, 89.14, 89.44, 88.43], dtype=np.float64)
/// real2 = np.array([91.59, 91.06, 92.87, 93.00, 93.61, 93.15, 92.84, 93.99, 94.55, 94.36, 95.53, 96.54, 96.89, 97.77, 97.29, 98.15, 99.44, 99.18, 98.67, 97.96, 99.43, 100.17, 100.78, 101.04, 100.58, 101.32, 100.13, 99.14, 99.44, 98.43], dtype=np.float64)
/// real3 = np.array([101.59, 101.06, 102.87, 103.00, 103.61, 103.15, 102.84, 103.99, 104.55, 104.36, 105.53, 106.54, 106.89, 107.77, 107.29, 108.15, 109.44, 109.18, 108.67, 107.96, 109.43, 110.17, 110.78, 111.04, 110.58, 111.32, 110.13, 109.14, 109.44, 108.43], dtype=np.float64)
/// real4 = np.array([111.59, 111.06, 112.87, 113.00, 113.61, 113.15, 112.84, 113.99, 114.55, 114.36, 115.53, 116.54, 116.89, 117.77, 117.29, 118.15, 119.44, 119.18, 118.67, 117.96, 119.43, 120.17, 120.78, 121.04, 120.58, 121.32, 120.13, 119.14, 119.44, 118.43], dtype=np.float64)
///
/// # Prepare inputs for SIMD processing (must be exactly 2, 4, 8, or 16 assets)
/// inputs = [
///     [real1],  # Asset 1
///     [real2],  # Asset 2
///     [real3],  # Asset 3
///     [real4],  # Asset 4
/// ]
///
/// # Calculate TRIX for all assets using SIMD
/// outputs, states = ti.indicators.trix.simd_by_assets(inputs, [14.0], None)
///
/// # outputs[0] contains TRIX values for asset 1
/// # outputs[1] contains TRIX values for asset 2
/// # outputs[2] contains TRIX values for asset 3
/// # outputs[3] contains TRIX values for asset 4
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
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<TrixState>)> {
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
        if asset_inputs.len() != rust_trix::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_trix::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_trix::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_trix::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_trix::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; rust_trix::INPUTS_WIDTH] = [
            asset_inputs[0].as_slice()?, // real
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_trix::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: [f64; rust_trix::OPTIONS_WIDTH] = [options[0]];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_trix::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_trix::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_trix::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_trix::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_trix::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_trix::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_trix::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_trix::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let trix_states: Vec<TrixState> = states
                .into_iter()
                .map(|state| TrixState { inner: state })
                .collect();
            Ok((results, trix_states))
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
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<TrixState>)> {
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

    if inputs.len() != rust_trix::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_trix::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != rust_trix::OPTIONS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                rust_trix::OPTIONS_WIDTH,
                opt.len()
            )));
        }
    }

    let input_arrays: [&[f64]; rust_trix::INPUTS_WIDTH] = [
        inputs[0].as_slice()?
    ];

    let mut option_arrays: Vec<[f64; rust_trix::OPTIONS_WIDTH]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0]]);
    }

    let option_refs: Vec<&[f64; rust_trix::OPTIONS_WIDTH]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; rust_trix::OPTIONS_WIDTH]; 2] =
                option_refs.as_slice().try_into().unwrap();
            rust_trix::by_options::indicator::<2>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let opt_array: &[&[f64; rust_trix::OPTIONS_WIDTH]; 4] =
                option_refs.as_slice().try_into().unwrap();
            rust_trix::by_options::indicator::<4>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let opt_array: &[&[f64; rust_trix::OPTIONS_WIDTH]; 8] =
                option_refs.as_slice().try_into().unwrap();
            rust_trix::by_options::indicator::<8>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let opt_array: &[&[f64; rust_trix::OPTIONS_WIDTH]; 16] =
                option_refs.as_slice().try_into().unwrap();
            rust_trix::by_options::indicator::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let trix_states: Vec<TrixState> = states
                .into_iter()
                .map(|state| TrixState { inner: state })
                .collect();
            Ok((results, trix_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the TRIX indicator module with Python
///
/// This function creates a Python submodule for the TRIX indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_trix_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "trix")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<TrixState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}

/// Get minimum data length required for TRIX calculation
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "TRIX requires exactly 1 option: period",
        ));
    }
    Ok(rust_trix::min_data(&options))
}

/// Get minimum data length required for TRIX calculation with accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "TRIX requires exactly 1 option: period",
        ));
    }
    Ok(rust_trix::min_data_accuracy(&options, decimals))
}

/// Get output length for TRIX calculation
#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "TRIX requires exactly 1 option: period",
        ));
    }
    Ok(rust_trix::output_length(data_len, &options))
}
