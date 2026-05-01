use crate::utils::info_to_hashmap;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::ad as rust_ad;

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct AdState {
    inner: rust_ad::IndicatorState,
}

#[pymethods]
impl AdState {
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_ad::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Expected {} inputs, got {}",
                rust_ad::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        let input_arrays: [&[f64]; rust_ad::INPUTS_WIDTH] = [
            inputs[0].as_slice()?,
            inputs[1].as_slice()?,
            inputs[2].as_slice()?,
            inputs[3].as_slice()?,
        ];

        match self
            .inner
            .batch_indicator(&input_arrays, optional_outputs.as_deref())
        {
            Ok(result) => Ok(result),
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

/// Register the AD indicator module with Python
///
/// This function creates a Python submodule for the AD indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_ad_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "ad")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_class::<AdState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, AdState)> {
    if inputs.len() != rust_ad::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_ad::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    if options.len() != rust_ad::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_ad::OPTIONS_WIDTH,
            options.len()
        )));
    }

    let input_arrays: [&[f64]; rust_ad::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
        inputs[3].as_slice()?,
    ];

    let options_array: [f64; rust_ad::OPTIONS_WIDTH] = [];

    match rust_ad::indicator(&input_arrays, &options_array, optional_outputs.as_deref()) {
        Ok((result, state)) => Ok((result, AdState { inner: state })),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Indicator calculation failed: {:?}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = rust_ad::info();
    Ok(info_to_hashmap(info))
}

#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    Ok(rust_ad::min_data(&options))
}

#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    Ok(rust_ad::min_data_accuracy(&options, decimals))
}

#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    Ok(rust_ad::output_length(data_len, &options))
}

/// Calculate AD (Accumulation/Distribution) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Accumulation/Distribution line is a volume-based indicator that measures
/// the cumulative flow of money into and out of a security. It uses the relationship
/// between the stock's close and its high-low range, combined with volume.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [high, low, close, volume] arrays
/// - options: Empty vector (AD requires no options)
/// - optional_outputs: Optional list of booleans for additional outputs (none available for AD)
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of AD results for each asset (each asset returns one AD line)
///   - states: Vector of AdState objects for continuing calculations
///
/// Input Structure:
/// The inputs parameter should be structured as:
/// ```
/// inputs = [
///     [high_asset1, low_asset1, close_asset1, volume_asset1],  # Asset 1
///     [high_asset2, low_asset2, close_asset2, volume_asset2],  # Asset 2
///     # ... more assets
/// ]
/// ```
///
/// Example:
/// ```python
/// import numpy as np
/// import tulip_rs as ti
///
/// # Data for 4 assets, 5 periods each (SIMD requires 2, 4, 8, or 16 assets)
/// high1 = np.array([10.5, 10.8, 11.0, 10.9, 11.2], dtype=np.float64)
/// low1 = np.array([10.0, 10.2, 10.5, 10.3, 10.8], dtype=np.float64)
/// close1 = np.array([10.3, 10.6, 10.8, 10.7, 11.0], dtype=np.float64)
/// volume1 = np.array([1000, 1200, 1500, 1100, 1300], dtype=np.float64)
///
/// high2 = np.array([20.5, 20.8, 21.0, 20.9, 21.2], dtype=np.float64)
/// low2 = np.array([20.0, 20.2, 20.5, 20.3, 20.8], dtype=np.float64)
/// close2 = np.array([20.3, 20.6, 20.8, 20.7, 21.0], dtype=np.float64)
/// volume2 = np.array([2000, 2200, 2500, 2100, 2300], dtype=np.float64)
///
/// high3 = np.array([30.5, 30.8, 31.0, 30.9, 31.2], dtype=np.float64)
/// low3 = np.array([30.0, 30.2, 30.5, 30.3, 30.8], dtype=np.float64)
/// close3 = np.array([30.3, 30.6, 30.8, 30.7, 31.0], dtype=np.float64)
/// volume3 = np.array([3000, 3200, 3500, 3100, 3300], dtype=np.float64)
///
/// high4 = np.array([40.5, 40.8, 41.0, 40.9, 41.2], dtype=np.float64)
/// low4 = np.array([40.0, 40.2, 40.5, 40.3, 40.8], dtype=np.float64)
/// close4 = np.array([40.3, 40.6, 40.8, 40.7, 41.0], dtype=np.float64)
/// volume4 = np.array([4000, 4200, 4500, 4100, 4300], dtype=np.float64)
///
/// # Prepare inputs for SIMD processing (must be exactly 2, 4, 8, or 16 assets)
/// inputs = [
///     [high1, low1, close1, volume1],  # Asset 1
///     [high2, low2, close2, volume2],  # Asset 2
///     [high3, low3, close3, volume3],  # Asset 3
///     [high4, low4, close4, volume4],  # Asset 4
/// ]
///
/// # Calculate AD for all assets using SIMD
/// outputs, states = ti.indicators.ad_simd_by_assets(inputs, [], None)
///
/// # outputs[0] contains AD values for asset 1
/// # outputs[1] contains AD values for asset 2
/// # outputs[2] contains AD values for asset 3
/// # outputs[3] contains AD values for asset 4
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
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<AdState>)> {
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
        if asset_inputs.len() != rust_ad::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_ad::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_ad::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_ad::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_ad::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; rust_ad::INPUTS_WIDTH] = [
            asset_inputs[0].as_slice()?, // high
            asset_inputs[1].as_slice()?, // low
            asset_inputs[2].as_slice()?, // close
            asset_inputs[3].as_slice()?, // volume
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_ad::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: [f64; rust_ad::OPTIONS_WIDTH] = [];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_ad::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_ad::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_ad::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_ad::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_ad::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_ad::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_ad::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_ad::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let ad_states: Vec<AdState> = states
                .into_iter()
                .map(|state| AdState { inner: state })
                .collect();
            Ok((results, ad_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}
