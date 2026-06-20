use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::rsi as rust_rsi;

/// RSI State wrapper for Python
#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct RsiState {
    inner: rust_rsi::IndicatorState,
}

#[pymethods]
impl RsiState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "RSI State - internal state for Relative Strength Index".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for RSI: just one array of real values)
    ///
    /// Returns:
    ///     List of output arrays (for RSI: just one array)
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        py: Python<'_>,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Py<PyArray1<f64>>>> {
        if inputs.len() != rust_rsi::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "RSI requires {} input arrays, got {}",
                rust_rsi::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (RSI only takes 1 input)
        let inputs_array: [&[f64]; rust_rsi::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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
        "RsiState(internal)".to_string()
    }
}

/// Relative Strength Index - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 1], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for RSI: just one array of real values)
///     options: Array of options (for RSI: just the period)
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (RSI has 1 output array)
///     - state: RsiState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> real = np.array([44, 44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 46.08, 45.89, 46.03, 46.83, 46.69, 46.45, 46.59, 46.3, 46.02, 46.74, 46.93, 47.37], dtype=np.float64)
///     >>> inputs = [real]  # RSI takes 1 input array
///     >>> options = [14.0]  # period = 14
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(outputs[0])  # RSI values
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Py<PyArray1<f64>>>, RsiState)> {
    // Validate inputs count
    if inputs.len() != rust_rsi::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "RSI requires {} input arrays, got {}",
            rust_rsi::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != rust_rsi::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_rsi::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Validate period
    if options[0] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Period must be >= 1",
        ));
    }

    // Direct extraction for single input (RSI only takes 1 input)
    let inputs_array: [&[f64]; rust_rsi::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    // Convert options to fixed-size array
    let options_array: [f64; rust_rsi::OPTIONS_WIDTH] = [options[0]];

    match rust_rsi::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = RsiState { inner: state };
            Ok((crate::utils::vecs_to_pyarrays(py, outputs), py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "RSI calculation error: {}",
            e
        ))),
    }
}

/// Get RSI info
#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_rsi::INFO)
}

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_options(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<Vec<f64>>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<RsiState>)> {
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

    if inputs.len() != rust_rsi::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_rsi::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != rust_rsi::OPTIONS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                rust_rsi::OPTIONS_WIDTH,
                opt.len()
            )));
        }
    }

    let input_arrays: [&[f64]; rust_rsi::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    let mut option_arrays: Vec<[f64; rust_rsi::OPTIONS_WIDTH]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0]]);
    }

    let option_refs: Vec<&[f64; rust_rsi::OPTIONS_WIDTH]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; rust_rsi::OPTIONS_WIDTH]; 2] =
                option_refs.as_slice().try_into().unwrap();
            rust_rsi::by_options::indicator::<2>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let opt_array: &[&[f64; rust_rsi::OPTIONS_WIDTH]; 4] =
                option_refs.as_slice().try_into().unwrap();
            rust_rsi::by_options::indicator::<4>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let opt_array: &[&[f64; rust_rsi::OPTIONS_WIDTH]; 8] =
                option_refs.as_slice().try_into().unwrap();
            rust_rsi::by_options::indicator::<8>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let opt_array: &[&[f64; rust_rsi::OPTIONS_WIDTH]; 16] =
                option_refs.as_slice().try_into().unwrap();
            rust_rsi::by_options::indicator::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let rsi_states: Vec<RsiState> = states
                .into_iter()
                .map(|state| RsiState { inner: state })
                .collect();
            Ok((crate::utils::simd_vecs_to_pyarrays(py, results), rsi_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the RSI indicator module with Python
///
/// This function creates a Python submodule for the RSI indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_rsi_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "rsi")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<RsiState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_rsi::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_rsi::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_rsi::min_data(&options))
}



/// Calculate RSI (Relative Strength Index) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Relative Strength Index (RSI) is a momentum oscillator that measures the
/// speed and change of price movements. It oscillates between 0 and 100 and is
/// typically used to identify overbought or oversold conditions.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [real] arrays
/// - options: Vector containing [period] for the RSI calculation
/// - optional_outputs: Optional list of booleans for additional outputs (none available for RSI)
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of RSI results for each asset (each asset returns one RSI line)
///   - states: Vector of RsiState objects for continuing calculations
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
/// # Data for 4 assets, 20 periods each (SIMD requires 2, 4, 8, or 16 assets)
/// real1 = np.array([44.0, 44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 46.08, 45.89,
///                   46.03, 46.83, 46.69, 46.45, 46.59, 46.30, 46.02, 46.74, 46.93, 47.37], dtype=np.float64)
///
/// real2 = np.array([54.0, 54.34, 54.09, 54.15, 53.61, 54.33, 54.83, 55.85, 56.08, 55.89,
///                   56.03, 56.83, 56.69, 56.45, 56.59, 56.30, 56.02, 56.74, 56.93, 57.37], dtype=np.float64)
///
/// real3 = np.array([64.0, 64.34, 64.09, 64.15, 63.61, 64.33, 64.83, 65.85, 66.08, 65.89,
///                   66.03, 66.83, 66.69, 66.45, 66.59, 66.30, 66.02, 66.74, 66.93, 67.37], dtype=np.float64)
///
/// real4 = np.array([74.0, 74.34, 74.09, 74.15, 73.61, 74.33, 74.83, 75.85, 76.08, 75.89,
///                   76.03, 76.83, 76.69, 76.45, 76.59, 76.30, 76.02, 76.74, 76.93, 77.37], dtype=np.float64)
///
/// # Prepare inputs for SIMD processing (must be exactly 2, 4, 8, or 16 assets)
/// inputs = [
///     [real1],  # Asset 1
///     [real2],  # Asset 2
///     [real3],  # Asset 3
///     [real4],  # Asset 4
/// ]
///
/// # RSI options: [period]
/// options = [14]  # 14-period RSI
///
/// # Calculate RSI for all assets using SIMD
/// outputs, states = ti.indicators.rsi_simd_by_assets(inputs, options, None)
///
/// # outputs[0] contains RSI values for asset 1
/// # outputs[1] contains RSI values for asset 2
/// # outputs[2] contains RSI values for asset 3
/// # outputs[3] contains RSI values for asset 4
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
    py: Python<'_>,
    inputs: Vec<Vec<PyReadonlyArray1<f64>>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<RsiState>)> {
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
        if asset_inputs.len() != rust_rsi::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_rsi::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_rsi::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_rsi::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_rsi::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; rust_rsi::INPUTS_WIDTH] = [
            asset_inputs[0].as_slice()?, // real
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_rsi::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: [f64; rust_rsi::OPTIONS_WIDTH] = [options[0]];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_rsi::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_rsi::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_rsi::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_rsi::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_rsi::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_rsi::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_rsi::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_rsi::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let rsi_states: Vec<RsiState> = states
                .into_iter()
                .map(|state| RsiState { inner: state })
                .collect();
            Ok((crate::utils::simd_vecs_to_pyarrays(py, results), rsi_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}
