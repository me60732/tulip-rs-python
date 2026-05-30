use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::sma as rust_sma;

/// SMA State wrapper for Python
#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct SmaState {
    inner: rust_sma::IndicatorState,
}

#[pymethods]
impl SmaState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "SMA State - internal state for Simple Moving Average".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for SMA: just one array of real values)
    ///
    /// Returns:
    ///     List of output arrays (for SMA: just one array)
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_sma::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "SMA requires {} input arrays, got {}",
                rust_sma::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (SMA only takes 1 input)
        let inputs_array: [&[f64]; rust_sma::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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
        "SmaState(internal)".to_string()
    }
}

/// Simple Moving Average - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 1], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for SMA: just one array of real values)
///     options: Array of options (for SMA: just the period)
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (SMA has 1 output array)
///     - state: SmaState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> real = np.array([1, 2, 3, 4, 5], dtype=np.float64)
///     >>> inputs = [real]  # SMA takes 1 input array
///     >>> options = [3.0]  # period = 3
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(outputs[0])  # [2.0, 3.0, 4.0]
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, SmaState)> {
    // Validate inputs count
    if inputs.len() != rust_sma::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "SMA requires {} input arrays, got {}",
            rust_sma::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != rust_sma::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_sma::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Validate period
    if options[0] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Period must be >= 1",
        ));
    }

    // Direct extraction for single input (SMA only takes 1 input)
    let inputs_array: [&[f64]; rust_sma::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    // Convert options to fixed-size array
    let options_array: [f64; rust_sma::OPTIONS_WIDTH] = [options[0]];

    match rust_sma::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = SmaState { inner: state };
            Ok((outputs, py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "SMA calculation error: {}",
            e
        ))),
    }
}

/// Get SMA info
#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_sma::INFO)
}


#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_options(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<Vec<f64>>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<SmaState>)> {
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

    if inputs.len() != rust_sma::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_sma::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != rust_sma::OPTIONS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                rust_sma::OPTIONS_WIDTH,
                opt.len()
            )));
        }
    }

    let input_arrays: [&[f64]; rust_sma::INPUTS_WIDTH] = [
        inputs[0].as_slice()?
    ];

    let mut option_arrays: Vec<[f64; rust_sma::OPTIONS_WIDTH]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0]]);
    }

    let option_refs: Vec<&[f64; rust_sma::OPTIONS_WIDTH]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; rust_sma::OPTIONS_WIDTH]; 2] =
                option_refs.as_slice().try_into().unwrap();
            rust_sma::by_options::indicator::<2>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let opt_array: &[&[f64; rust_sma::OPTIONS_WIDTH]; 4] =
                option_refs.as_slice().try_into().unwrap();
            rust_sma::by_options::indicator::<4>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let opt_array: &[&[f64; rust_sma::OPTIONS_WIDTH]; 8] =
                option_refs.as_slice().try_into().unwrap();
            rust_sma::by_options::indicator::<8>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let opt_array: &[&[f64; rust_sma::OPTIONS_WIDTH]; 16] =
                option_refs.as_slice().try_into().unwrap();
            rust_sma::by_options::indicator::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let sma_states: Vec<SmaState> = states
                .into_iter()
                .map(|state| SmaState { inner: state })
                .collect();
            Ok((results, sma_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the SMA indicator module with Python
///
/// This function creates a Python submodule for the SMA indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_sma_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "sma")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<SmaState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_sma::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_sma::OPTIONS_WIDTH,
            options.len()
        )));
    }
    let options_array: [f64; rust_sma::OPTIONS_WIDTH] = [options[0]];
    Ok(rust_sma::min_data(&options_array))
}

/// Get expected output length
#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_sma::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_sma::OPTIONS_WIDTH,
            options.len()
        )));
    }
    let options_array: [f64; rust_sma::OPTIONS_WIDTH] = [options[0]];
    Ok(rust_sma::output_length(data_length, &options_array))
}

/// Get minimum data required for accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != rust_sma::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_sma::OPTIONS_WIDTH,
            options.len()
        )));
    }
    let options_array: [f64; rust_sma::OPTIONS_WIDTH] = [options[0]];
    Ok(rust_sma::min_data_accuracy(&options_array, decimals))
}

/// Calculate SMA (Simple Moving Average) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Simple Moving Average (SMA) calculates the arithmetic mean of a given set
/// of values over a specified number of periods. It's one of the most basic and
/// widely used technical indicators.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [real] arrays
/// - options: Vector containing [period] for the SMA calculation
/// - optional_outputs: Optional list of booleans for additional outputs (none available for SMA)
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of SMA results for each asset (each asset returns one SMA line)
///   - states: Vector of SmaState objects for continuing calculations
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
/// # Data for 4 assets, 10 periods each (SIMD requires 2, 4, 8, or 16 assets)
/// real1 = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], dtype=np.float64)
/// real2 = np.array([11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0], dtype=np.float64)
/// real3 = np.array([21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0], dtype=np.float64)
/// real4 = np.array([31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0], dtype=np.float64)
///
/// # Prepare inputs for SIMD processing (must be exactly 2, 4, 8, or 16 assets)
/// inputs = [
///     [real1],  # Asset 1
///     [real2],  # Asset 2
///     [real3],  # Asset 3
///     [real4],  # Asset 4
/// ]
///
/// # SMA options: [period]
/// options = [3]  # 3-period SMA
///
/// # Calculate SMA for all assets using SIMD
/// outputs, states = ti.indicators.sma_simd_by_assets(inputs, options, None)
///
/// # outputs[0] contains SMA values for asset 1
/// # outputs[1] contains SMA values for asset 2
/// # outputs[2] contains SMA values for asset 3
/// # outputs[3] contains SMA values for asset 4
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
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<SmaState>)> {
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
        if asset_inputs.len() != rust_sma::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_sma::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_sma::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_sma::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_sma::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; rust_sma::INPUTS_WIDTH] = [
            asset_inputs[0].as_slice()?, // real
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_sma::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: [f64; rust_sma::OPTIONS_WIDTH] = [options[0]];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_sma::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_sma::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_sma::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_sma::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_sma::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_sma::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_sma::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_sma::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let sma_states: Vec<SmaState> = states
                .into_iter()
                .map(|state| SmaState { inner: state })
                .collect();
            Ok((results, sma_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}
