use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::HashMap;
use tulip_rs::indicators::adx::{Adx, IndicatorState, INPUTS, OPTIONS, TIndicatorState, IndicatorByOptions, Indicator};

/// ADX State wrapper for Python
#[pyclass]
pub struct AdxState {
    inner: IndicatorState,
}

#[pymethods]
impl AdxState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "ADX State - internal state for Average Directional Index".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays [high, low, close]
    ///
    /// Returns:
    ///     List of output arrays [adx] + optional outputs [dx, atr, tr]
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        py: Python<'_>,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Py<PyArray1<f64>>>> {
        if inputs.len() != INPUTS {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "ADX requires {} input arrays, got {}",
                INPUTS,
                inputs.len()
            )));
        }

        // Direct extraction for three inputs (high, low, close)
        let inputs_array: [&[f64]; INPUTS] = [
            inputs[0].as_slice()?,
            inputs[1].as_slice()?,
            inputs[2].as_slice()?,
        ];

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
}

/// Calculate ADX (Average Directional Index)
///
/// The Average Directional Index (ADX) is a trend strength indicator that measures
/// the strength of a trend without regard to its direction.
///
/// Parameters:
/// - inputs: List of numpy arrays [high, low, close]
/// - options: List containing [period]
/// - optional_outputs: Optional list of booleans for additional outputs [dx, atr, tr]
///
/// Returns:
/// - Tuple of (outputs, state) where outputs is [adx_line] + optional outputs
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Py<PyArray1<f64>>>, AdxState)> {
    if options.len() != OPTIONS {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            OPTIONS,
            options.len()
        )));
    }

    if inputs.len() != INPUTS {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "ADX requires {} input arrays, got {}",
            INPUTS,
            inputs.len()
        )));
    }

    // Direct extraction for three inputs (high, low, close)
    let inputs_array: [&[f64]; INPUTS] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];
    let options_array: [f64; OPTIONS] = [options[0]];

    match Adx::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => Ok((
            crate::utils::vecs_to_pyarrays(py, outputs),
            AdxState { inner: state },
        )),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Calculation error: {}",
            e
        ))),
    }
}

/// Get ADX indicator information
#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, Adx::INFO)
}

/// Get minimum data length required for ADX calculation
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    let options_array: [f64; OPTIONS] = [options[0]];
    Ok(Adx::min_data(&options_array))
}

/// Calculate ADX (Average Directional Index) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Average Directional Index (ADX) is a trend strength indicator that measures
/// the strength of a trend without regard to its direction. It ranges from 0 to 100,
/// where higher values indicate stronger trends.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [high, low, close] arrays
/// - options: Vector containing [period] for the ADX calculation
/// - optional_outputs: Optional list of booleans for additional outputs [dx, atr, tr]
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of ADX results for each asset (each asset returns [adx] + optional outputs)
///   - states: Vector of AdxState objects for continuing calculations
///
/// Input Structure:
/// The inputs parameter should be structured as:
/// ```
/// inputs = [
///     [high_asset1, low_asset1, close_asset1],  # Asset 1
///     [high_asset2, low_asset2, close_asset2],  # Asset 2
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
/// close1 = np.array([81.59, 81.06, 82.87, 83.00, 83.61, 83.15, 82.84, 83.99, 84.55, 84.36, 85.53, 86.54, 86.89, 87.77, 87.29, 88.1, 88.5, 89.1, 89.9, 90.2], dtype=np.float64)
///
/// # Similar data for assets 2, 3, 4...
///
/// # Prepare inputs for SIMD processing (must be exactly 2, 4, 8, or 16 assets)
/// inputs = [
///     [high1, low1, close1],  # Asset 1
///     [high2, low2, close2],  # Asset 2
///     [high3, low3, close3],  # Asset 3
///     [high4, low4, close4],  # Asset 4
/// ]
///
/// # ADX options: [period]
/// options = [14.0]  # 14-period ADX
///
/// # Calculate ADX for all assets using SIMD
/// outputs, states = ti.indicators.adx_simd_by_assets(inputs, options, None)
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
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<AdxState>)> {
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
        let input_array: [&[f64]; INPUTS] = [
            asset_inputs[0].as_slice()?, // high
            asset_inputs[1].as_slice()?, // low
            asset_inputs[2].as_slice()?, // close
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; INPUTS]> = asset_input_arrays.iter().collect();

    let options_array: [f64; OPTIONS] = [options[0]];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; INPUTS]; 2] = input_refs.as_slice().try_into().unwrap();
            Adx::indicator_by_assets::<2>(input_array, &options_array, optional_outputs.as_deref())
        }
        4 => {
            let input_array: &[&[&[f64]; INPUTS]; 4] = input_refs.as_slice().try_into().unwrap();
            Adx::indicator_by_assets::<4>(input_array, &options_array, optional_outputs.as_deref())
        }
        8 => {
            let input_array: &[&[&[f64]; INPUTS]; 8] = input_refs.as_slice().try_into().unwrap();
            Adx::indicator_by_assets::<8>(input_array, &options_array, optional_outputs.as_deref())
        }
        16 => {
            let input_array: &[&[&[f64]; INPUTS]; 16] = input_refs.as_slice().try_into().unwrap();
            Adx::indicator_by_assets::<16>(input_array, &options_array, optional_outputs.as_deref())
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let adx_states: Vec<AdxState> = states
                .into_iter()
                .map(|state| AdxState { inner: state })
                .collect();
            Ok((crate::utils::simd_vecs_to_pyarrays(py, results), adx_states))
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
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<Vec<f64>>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<AdxState>)> {
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

    let input_arrays: [&[f64]; INPUTS] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];

    let mut option_arrays: Vec<[f64; OPTIONS]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0]]);
    }

    let option_refs: Vec<&[f64; OPTIONS]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; OPTIONS]; 2] = option_refs.as_slice().try_into().unwrap();
            Adx::indicator_by_options::<2>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        4 => {
            let opt_array: &[&[f64; OPTIONS]; 4] = option_refs.as_slice().try_into().unwrap();
            Adx::indicator_by_options::<4>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        8 => {
            let opt_array: &[&[f64; OPTIONS]; 8] = option_refs.as_slice().try_into().unwrap();
            Adx::indicator_by_options::<8>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        16 => {
            let opt_array: &[&[f64; OPTIONS]; 16] = option_refs.as_slice().try_into().unwrap();
            Adx::indicator_by_options::<16>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let adx_states: Vec<AdxState> = states
                .into_iter()
                .map(|state| AdxState { inner: state })
                .collect();
            Ok((crate::utils::simd_vecs_to_pyarrays(py, results), adx_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the ADX indicator module with Python
///
/// This function creates a Python submodule for the ADX indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_adx_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "adx")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;

    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<AdxState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}
