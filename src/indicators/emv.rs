use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::emv as rust_emv;

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct EmvState {
    inner: rust_emv::IndicatorState,
}

#[pymethods]
impl EmvState {
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_emv::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Expected {} inputs, got {}",
                rust_emv::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        let input_arrays: [&[f64]; rust_emv::INPUTS_WIDTH] = [
            inputs[0].as_slice()?,
            inputs[1].as_slice()?,
            inputs[2].as_slice()?,
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

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, EmvState)> {
    if inputs.len() != rust_emv::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_emv::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    if options.len() != rust_emv::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_emv::OPTIONS_WIDTH,
            options.len()
        )));
    }

    let input_arrays: [&[f64]; rust_emv::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];

    let options_array: [f64; rust_emv::OPTIONS_WIDTH] = [];

    match rust_emv::indicator(&input_arrays, &options_array, optional_outputs.as_deref()) {
        Ok((result, state)) => Ok((result, EmvState { inner: state })),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Indicator calculation failed: {:?}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_emv::INFO)
}

#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    Ok(rust_emv::min_data(&options))
}

#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    Ok(rust_emv::min_data_accuracy(&options, decimals))
}

#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    Ok(rust_emv::output_length(data_len, &options))
}

/// Calculate EMV (Ease of Movement) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Ease of Movement (EMV) indicator combines price and volume information to
/// assess how easily a price can move. It identifies periods when prices are
/// advancing or declining with relative ease based on volume.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [high, low, volume] arrays
/// - options: Empty vector (EMV requires no options)
/// - optional_outputs: Optional list of booleans for additional outputs (none available for EMV)
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of EMV results for each asset (each asset returns one EMV line)
///   - states: Vector of EmvState objects for continuing calculations
///
/// Input Structure:
/// The inputs parameter should be structured as:
/// ```
/// inputs = [
///     [high_asset1, low_asset1, volume_asset1],  # Asset 1
///     [high_asset2, low_asset2, volume_asset2],  # Asset 2
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
/// high1 = np.array([10.5, 10.8, 11.0, 10.9, 11.2, 11.1, 11.3, 11.0, 10.8, 11.1, 11.4, 11.2, 11.5, 11.3, 11.6, 11.4, 11.7, 11.5, 11.8, 11.6], dtype=np.float64)
/// low1 = np.array([10.0, 10.2, 10.5, 10.3, 10.8, 10.7, 10.9, 10.6, 10.4, 10.7, 11.0, 10.8, 11.1, 10.9, 11.2, 11.0, 11.3, 11.1, 11.4, 11.2], dtype=np.float64)
/// volume1 = np.array([1000, 1200, 1500, 1100, 1300, 1400, 1600, 1200, 1000, 1350, 1700, 1500, 1800, 1600, 1900, 1750, 2000, 1850, 2100, 1950], dtype=np.float64)
///
/// # Similar data for assets 2, 3, 4...
///
/// # Prepare inputs for SIMD processing (must be exactly 2, 4, 8, or 16 assets)
/// inputs = [
///     [high1, low1, volume1],  # Asset 1
///     [high2, low2, volume2],  # Asset 2
///     [high3, low3, volume3],  # Asset 3
///     [high4, low4, volume4],  # Asset 4
/// ]
///
/// # EMV options: empty (no parameters needed)
/// options = []
///
/// # Calculate EMV for all assets using SIMD
/// outputs, states = ti.indicators.emv_simd_by_assets(inputs, options, None)
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
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<EmvState>)> {
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
        if asset_inputs.len() != rust_emv::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_emv::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_emv::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_emv::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_emv::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; rust_emv::INPUTS_WIDTH] = [
            asset_inputs[0].as_slice()?, // high
            asset_inputs[1].as_slice()?, // low
            asset_inputs[2].as_slice()?, // volume
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_emv::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: [f64; rust_emv::OPTIONS_WIDTH] = [];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_emv::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_emv::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_emv::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_emv::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_emv::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_emv::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_emv::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_emv::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let emv_states: Vec<EmvState> = states
                .into_iter()
                .map(|state| EmvState { inner: state })
                .collect();
            Ok((results, emv_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}

// Auto-register functions for EMV
pub fn register_emv_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "emv")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_class::<EmvState>()?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}
