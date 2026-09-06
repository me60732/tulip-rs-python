use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicators::ao::{
    Ao, Indicator, IndicatorState, TIndicatorState, INPUTS, OPTIONS,
};

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct AoState {
    inner: IndicatorState,
}

#[pymethods]
impl AoState {
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

        let input_arrays: [&[f64]; INPUTS] = [inputs[0].as_slice()?, inputs[1].as_slice()?];

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
) -> PyResult<(Vec<Py<PyArray1<f64>>>, AoState)> {
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

    let input_arrays: [&[f64]; INPUTS] = [inputs[0].as_slice()?, inputs[1].as_slice()?];

    let options_array: [f64; OPTIONS] = [];

    match Ao::indicator(&input_arrays, &options_array, optional_outputs.as_deref()) {
        Ok((result, state)) => Ok((
            crate::utils::vecs_to_pyarrays(py, result),
            AoState { inner: state },
        )),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Indicator calculation failed: {:?}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, Ao::INFO)
}

#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != OPTIONS {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            OPTIONS,
            options.len()
        )));
    }
    let options_array: [f64; OPTIONS] = [];
    Ok(Ao::min_data(&options_array))
}

/// Calculate AO (Awesome Oscillator) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Awesome Oscillator (AO) is a momentum indicator that measures the difference
/// between a 5-period and 34-period simple moving average of the midpoint price.
/// It helps identify trend changes and momentum shifts.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [high, low] arrays
/// - options: Empty vector (AO requires no options)
/// - optional_outputs: Optional list of booleans for additional outputs (none available for AO)
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of AO results for each asset (each asset returns one oscillator line)
///   - states: Vector of AoState objects for continuing calculations
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
/// # Data for 4 assets, 40 periods each (SIMD requires 2, 4, 8, or 16 assets)
/// high1 = np.array([10.5, 10.8, 11.0, 10.9, 11.2, 11.1, 11.3, 11.0, 10.8, 11.1, 11.4, 11.2, 11.5, 11.3, 11.6, 11.4, 11.7, 11.5, 11.8, 11.6, 11.9, 11.7, 12.0, 11.8, 12.1, 11.9, 12.2, 12.0, 12.3, 12.1, 12.4, 12.2, 12.5, 12.3, 12.6, 12.4, 12.7, 12.5, 12.8, 12.6], dtype=np.float64)
/// low1 = np.array([10.0, 10.2, 10.5, 10.3, 10.8, 10.7, 10.9, 10.6, 10.4, 10.7, 11.0, 10.8, 11.1, 10.9, 11.2, 11.0, 11.3, 11.1, 11.4, 11.2, 11.5, 11.3, 11.6, 11.4, 11.7, 11.5, 11.8, 11.6, 11.9, 11.7, 12.0, 11.8, 12.1, 11.9, 12.2, 12.0, 12.3, 12.1, 12.4, 12.2], dtype=np.float64)
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
/// # AO options: empty (no parameters needed)
/// options = []
///
/// # Calculate AO for all assets using SIMD
/// outputs, states = ti.indicators.ao_simd_by_assets(inputs, options, None)
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
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<AoState>)> {
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
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; INPUTS]> = asset_input_arrays.iter().collect();

    let options_array: [f64; OPTIONS] = [];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; INPUTS]; 2] = input_refs.as_slice().try_into().unwrap();
            Ao::indicator_by_assets::<2>(input_array, &options_array, optional_outputs.as_deref())
        }
        4 => {
            let input_array: &[&[&[f64]; INPUTS]; 4] = input_refs.as_slice().try_into().unwrap();
            Ao::indicator_by_assets::<4>(input_array, &options_array, optional_outputs.as_deref())
        }
        8 => {
            let input_array: &[&[&[f64]; INPUTS]; 8] = input_refs.as_slice().try_into().unwrap();
            Ao::indicator_by_assets::<8>(input_array, &options_array, optional_outputs.as_deref())
        }
        16 => {
            let input_array: &[&[&[f64]; INPUTS]; 16] = input_refs.as_slice().try_into().unwrap();
            Ao::indicator_by_assets::<16>(input_array, &options_array, optional_outputs.as_deref())
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let ao_states: Vec<AoState> = states
                .into_iter()
                .map(|state| AoState { inner: state })
                .collect();
            Ok((crate::utils::simd_vecs_to_pyarrays(py, results), ao_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the AO indicator module with Python
///
/// This function creates a Python submodule for the AO indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_ao_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "ao")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;

    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_class::<AoState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}
