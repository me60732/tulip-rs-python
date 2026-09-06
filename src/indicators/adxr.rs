use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicators::adxr::{Adxr, IndicatorState, INPUTS, OPTIONS, Indicator, TIndicatorState, IndicatorByOptions};

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct AdxrState {
    inner: IndicatorState,
}

#[pymethods]
impl AdxrState {
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        py: Python<'_>,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Py<PyArray1<f64>>>> {
        if inputs.len() != INPUTS {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Expected {} inputs, got {}",
                INPUTS,
                inputs.len()
            )));
        }

        let input_arrays: [&[f64]; INPUTS] = [
            inputs[0].as_slice()?,
            inputs[1].as_slice()?,
            inputs[2].as_slice()?,
        ];

        match self
            .inner
            .batch_indicator(&input_arrays, optional_outputs.as_deref())
        {
            Ok(result) => Ok(crate::utils::vecs_to_pyarrays(py, result)),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Indicator calculation failed: {:?}",
                e
            ))),
        }
    }

    fn __getstate__(&self) -> PyResult<HashMap<String, String>> {
        let serialized = serde_json::to_string(&self.inner).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Serialization failed: {}",
                e
            ))
        })?;
        let mut state = HashMap::new();
        state.insert("inner".to_string(), serialized);
        Ok(state)
    }

    fn __setstate__(&mut self, state: HashMap<String, String>) -> PyResult<()> {
        if let Some(inner_str) = state.get("inner") {
            self.inner = serde_json::from_str(inner_str).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Deserialization failed: {}",
                    e
                ))
            })?;
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Missing 'inner' key in state",
            ))
        }
    }
}

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Py<PyArray1<f64>>>, AdxrState)> {
    if inputs.len() != INPUTS {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            INPUTS,
            inputs.len()
        )));
    }

    if options.len() != OPTIONS {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            OPTIONS,
            options.len()
        )));
    }

    let input_arrays: [&[f64]; INPUTS] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];

    let options_array: [f64; OPTIONS] = [options[0]];

    match Adxr::indicator(&input_arrays, &options_array, optional_outputs.as_deref()) {
        Ok((result, state)) => Ok((
            crate::utils::vecs_to_pyarrays(py, result),
            AdxrState { inner: state },
        )),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Indicator calculation failed: {:?}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, Adxr::INFO)
}

#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    let options_array: [f64; OPTIONS] = [options[0]];
    Ok(Adxr::min_data(&options_array))
}

/// Calculate ADXR (Average Directional Index Rating) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Average Directional Index Rating (ADXR) is a smoothed version of ADX that provides
/// a more stable reading of trend strength by averaging the current ADX with the ADX
/// from a specified number of periods ago.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [high, low, close] arrays
/// - options: Vector containing [period] for the ADXR calculation
/// - optional_outputs: Optional list of booleans for additional outputs
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of ADXR results for each asset (each asset returns one ADXR line)
///   - states: Vector of AdxrState objects for continuing calculations
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
/// # Data for 4 assets, 25 periods each (SIMD requires 2, 4, 8, or 16 assets)
/// high1 = np.array([82.15, 81.89, 83.03, 83.30, 83.85, 83.90, 83.33, 84.30, 84.84, 85.00, 85.90, 86.58, 86.98, 88.00, 87.87, 88.32, 88.76, 89.25, 90.1, 90.5, 91.2, 91.8, 92.1, 92.5, 93.0], dtype=np.float64)
/// low1 = np.array([81.29, 80.64, 81.31, 82.65, 83.07, 83.11, 82.49, 82.30, 84.15, 84.11, 84.03, 85.39, 85.76, 87.17, 87.01, 87.5, 88.1, 88.9, 89.2, 89.8, 90.5, 91.1, 91.5, 91.9, 92.3], dtype=np.float64)
/// close1 = np.array([81.59, 81.06, 82.87, 83.00, 83.61, 83.15, 82.84, 83.99, 84.55, 84.36, 85.53, 86.54, 86.89, 87.77, 87.29, 88.1, 88.5, 89.1, 89.9, 90.2, 90.8, 91.4, 91.8, 92.2, 92.7], dtype=np.float64)
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
/// # ADXR options: [period]
/// options = [14.0]  # 14-period ADXR
///
/// # Calculate ADXR for all assets using SIMD
/// outputs, states = ti.indicators.adxr_simd_by_assets(inputs, options, None)
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
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<AdxrState>)> {
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
            Adxr::indicator_by_assets::<2>(input_array, &options_array, optional_outputs.as_deref())
        }
        4 => {
            let input_array: &[&[&[f64]; INPUTS]; 4] = input_refs.as_slice().try_into().unwrap();
            Adxr::indicator_by_assets::<4>(input_array, &options_array, optional_outputs.as_deref())
        }
        8 => {
            let input_array: &[&[&[f64]; INPUTS]; 8] = input_refs.as_slice().try_into().unwrap();
            Adxr::indicator_by_assets::<8>(input_array, &options_array, optional_outputs.as_deref())
        }
        16 => {
            let input_array: &[&[&[f64]; INPUTS]; 16] = input_refs.as_slice().try_into().unwrap();
            Adxr::indicator_by_assets::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let adxr_states: Vec<AdxrState> = states
                .into_iter()
                .map(|state| AdxrState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                adxr_states,
            ))
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
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<AdxrState>)> {
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
            Adxr::indicator_by_options::<2>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        4 => {
            let opt_array: &[&[f64; OPTIONS]; 4] = option_refs.as_slice().try_into().unwrap();
            Adxr::indicator_by_options::<4>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        8 => {
            let opt_array: &[&[f64; OPTIONS]; 8] = option_refs.as_slice().try_into().unwrap();
            Adxr::indicator_by_options::<8>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        16 => {
            let opt_array: &[&[f64; OPTIONS]; 16] = option_refs.as_slice().try_into().unwrap();
            Adxr::indicator_by_options::<16>(&input_arrays, opt_array, optional_outputs.as_deref())
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let adxr_states: Vec<AdxrState> = states
                .into_iter()
                .map(|state| AdxrState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                adxr_states,
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the ADXR indicator module with Python
///
/// This function creates a Python submodule for the ADXR indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_adxr_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "adxr")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;

    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<AdxrState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}
