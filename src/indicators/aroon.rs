use crate::utils::info_to_hashmap;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::aroon as rust_aroon;

/// AROON State wrapper for Python
#[pyclass]
pub struct AroonState {
    inner: rust_aroon::IndicatorState,
}

#[pymethods]
impl AroonState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "AROON State - internal state for Aroon Trend Indicator".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays [high, low]
    ///
    /// Returns:
    ///     List of output arrays [aroon_down, aroon_up]
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_aroon::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "AROON requires {} input arrays, got {}",
                rust_aroon::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for two inputs (high, low)
        let inputs_array: [&[f64]; rust_aroon::INPUTS_WIDTH] =
            [inputs[0].as_slice()?, inputs[1].as_slice()?];

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

/// Calculate AROON (Aroon Trend Indicator)
///
/// The Aroon indicator is used to identify trend changes and the strength of a trend.
/// It consists of Aroon Up and Aroon Down lines that oscillate between 0 and 100.
///
/// Parameters:
/// - inputs: List of numpy arrays [high, low]
/// - options: List containing [period]
/// - optional_outputs: Optional list of booleans for additional outputs (none available)
///
/// Returns:
/// - Tuple of (outputs, state) where outputs is [aroon_down, aroon_up]
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, AroonState)> {
    if options.len() != rust_aroon::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_aroon::OPTIONS_WIDTH,
            options.len()
        )));
    }

    if inputs.len() != rust_aroon::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "AROON requires {} input arrays, got {}",
            rust_aroon::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Direct extraction for two inputs (high, low)
    let inputs_array: [&[f64]; rust_aroon::INPUTS_WIDTH] =
        [inputs[0].as_slice()?, inputs[1].as_slice()?];
    let options_array: [f64; rust_aroon::OPTIONS_WIDTH] = [options[0]];

    match rust_aroon::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => Ok((outputs, AroonState { inner: state })),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Calculation error: {}",
            e
        ))),
    }
}

/// Get AROON indicator information
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = rust_aroon::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data length required for AROON calculation
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_aroon::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_aroon::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_aroon::min_data(&options))
}

/// Get minimum data length required for AROON calculation with accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != rust_aroon::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_aroon::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_aroon::min_data_accuracy(&options, decimals))
}

/// Get output length for AROON calculation
#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_aroon::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_aroon::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_aroon::output_length(data_len, &options))
}

/// Calculate AROON (Aroon Trend Indicator) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Aroon indicator identifies trend changes and the strength of a trend.
/// It consists of Aroon Up and Aroon Down lines that oscillate between 0 and 100.
/// Values near 100 indicate strong trends, while values near 0 indicate weak trends.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [high, low] arrays
/// - options: Vector containing [period] for the Aroon calculation
/// - optional_outputs: Optional list of booleans for additional outputs (none available for AROON)
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of AROON results for each asset (each asset returns [aroon_down, aroon_up])
///   - states: Vector of AroonState objects for continuing calculations
///
/// Input Structure:
/// The inputs parameter should be structured as:
/// ```
/// inputs = [
///     [high_asset1, low_asset1],  # Asset 1
///     [high_asset2, low_asset2],  # Asset 2
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
/// high1 = np.array([82.15, 81.89, 83.03, 83.30, 83.85, 83.90, 83.33, 84.30, 84.84, 85.00, 85.90, 86.58, 86.98, 88.00, 87.87, 88.32, 88.76, 89.25, 90.1, 90.5], dtype=np.float64)
/// low1 = np.array([81.29, 80.64, 81.31, 82.65, 83.07, 83.11, 82.49, 82.30, 84.15, 84.11, 84.03, 85.39, 85.76, 87.17, 87.01, 87.5, 88.1, 88.9, 89.2, 89.8], dtype=np.float64)
///
/// # Similar data for assets 2, 3, 4...
///
/// # Prepare inputs for SIMD processing (must be exactly 2, 4, 8, or 16 assets)
/// inputs = [
///     [high1, low1],  # Asset 1
///     [high2, low2],  # Asset 2
///     [high3, low3],  # Asset 3
///     [high4, low4],  # Asset 4
/// ]
///
/// # AROON options: [period]
/// options = [14.0]  # 14-period AROON
///
/// # Calculate AROON for all assets using SIMD
/// outputs, states = ti.indicators.aroon_simd_by_assets(inputs, options, None)
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
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<AroonState>)> {
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
        if asset_inputs.len() != rust_aroon::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_aroon::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_aroon::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_aroon::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_aroon::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; rust_aroon::INPUTS_WIDTH] = [
            asset_inputs[0].as_slice()?, // high
            asset_inputs[1].as_slice()?, // low
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_aroon::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: [f64; rust_aroon::OPTIONS_WIDTH] = [options[0]];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_aroon::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_aroon::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_aroon::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_aroon::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_aroon::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_aroon::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_aroon::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_aroon::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let aroon_states: Vec<AroonState> = states
                .into_iter()
                .map(|state| AroonState { inner: state })
                .collect();
            Ok((results, aroon_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the AROON indicator module with Python
///
/// This function creates a Python submodule for the AROON indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_aroon_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "aroon")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_class::<AroonState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}
