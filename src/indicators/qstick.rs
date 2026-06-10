use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::qstick as rust_qstick;

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct QstickState {
    inner: rust_qstick::IndicatorState,
}

#[pymethods]
impl QstickState {
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        py: Python<'_>,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Py<PyArray1<f64>>>> {
        if inputs.len() != rust_qstick::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Expected {} inputs, got {}",
                rust_qstick::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        let input_arrays: [&[f64]; rust_qstick::INPUTS_WIDTH] =
            [inputs[0].as_slice()?, inputs[1].as_slice()?];

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
) -> PyResult<(Vec<Py<PyArray1<f64>>>, QstickState)> {
    if inputs.len() != rust_qstick::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_qstick::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    if options.len() != rust_qstick::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_qstick::OPTIONS_WIDTH,
            options.len()
        )));
    }

    let input_arrays: [&[f64]; rust_qstick::INPUTS_WIDTH] =
        [inputs[0].as_slice()?, inputs[1].as_slice()?];

    let options_array: [f64; rust_qstick::OPTIONS_WIDTH] = [options[0]];

    match rust_qstick::indicator(&input_arrays, &options_array, optional_outputs.as_deref()) {
        Ok((result, state)) => Ok((
            crate::utils::vecs_to_pyarrays(py, result),
            QstickState { inner: state },
        )),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Indicator calculation failed: {:?}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_qstick::INFO)
}

#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    Ok(rust_qstick::min_data(&options))
}

#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    Ok(rust_qstick::min_data_accuracy(&options, decimals))
}

#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    Ok(rust_qstick::output_length(data_len, &options))
}

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_options(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<Vec<f64>>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<QstickState>)> {
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

    if inputs.len() != rust_qstick::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_qstick::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != rust_qstick::OPTIONS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                rust_qstick::OPTIONS_WIDTH,
                opt.len()
            )));
        }
    }

    let input_arrays: [&[f64]; rust_qstick::INPUTS_WIDTH] =
        [inputs[0].as_slice()?, inputs[1].as_slice()?];

    let mut option_arrays: Vec<[f64; rust_qstick::OPTIONS_WIDTH]> = Vec::with_capacity(num_options);

    for opt in &options {
        option_arrays.push([opt[0]]);
    }

    let option_refs: Vec<&[f64; rust_qstick::OPTIONS_WIDTH]> = option_arrays.iter().collect();

    // Call the SIMD by options function with proper const generic
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; rust_qstick::OPTIONS_WIDTH]; 2] =
                option_refs.as_slice().try_into().unwrap();
            rust_qstick::by_options::indicator::<2>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let opt_array: &[&[f64; rust_qstick::OPTIONS_WIDTH]; 4] =
                option_refs.as_slice().try_into().unwrap();
            rust_qstick::by_options::indicator::<4>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let opt_array: &[&[f64; rust_qstick::OPTIONS_WIDTH]; 8] =
                option_refs.as_slice().try_into().unwrap();
            rust_qstick::by_options::indicator::<8>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let opt_array: &[&[f64; rust_qstick::OPTIONS_WIDTH]; 16] =
                option_refs.as_slice().try_into().unwrap();
            rust_qstick::by_options::indicator::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let qstick_states: Vec<QstickState> = states
                .into_iter()
                .map(|state| QstickState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                qstick_states,
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

/// Register the QSTICK indicator module with Python
///
/// This function creates a Python submodule for the QSTICK indicator and registers
/// all its functions and classes.
///
/// # Arguments
/// * `parent_module` - The parent module to register this indicator under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration
pub fn register_qstick_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "qstick")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;

    submodule.add_class::<QstickState>()?;

    parent_module.add_submodule(&submodule)?;

    Ok(())
}

/// Calculate QSTICK (Qstick) for multiple assets using SIMD operations
///
/// This function processes multiple assets simultaneously for improved performance
/// using SIMD (Single Instruction, Multiple Data) operations.
///
/// The Qstick indicator is a technical analysis indicator that attempts to quantify
/// buying and selling pressure by looking at the relationship between the opening
/// and closing prices of a security over a specified period.
///
/// Parameters:
/// - inputs: Vector of asset inputs, where each asset contains [open, close] arrays
/// - options: Vector containing [period] for the QSTICK calculation
/// - optional_outputs: Optional list of booleans for additional outputs (none available for QSTICK)
///
/// Returns:
/// - Tuple of (outputs, states) where:
///   - outputs: Vector of QSTICK results for each asset (each asset returns one QSTICK line)
///   - states: Vector of QstickState objects for continuing calculations
///
/// Input Structure:
/// The inputs parameter should be structured as:
/// ```
/// inputs = [
///     [open_asset1, close_asset1],  # Asset 1
///     [open_asset2, close_asset2],  # Asset 2
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
/// open1 = np.array([10.0, 10.2, 10.5, 10.3, 10.8, 11.0, 10.9, 11.2, 11.1, 11.4,
///                   11.3, 11.6, 11.5, 11.8, 11.7, 12.0, 11.9, 12.2, 12.1, 12.4], dtype=np.float64)
/// close1 = np.array([10.3, 10.6, 10.8, 10.7, 11.0, 11.2, 11.1, 11.4, 11.3, 11.6,
///                    11.5, 11.8, 11.7, 12.0, 11.9, 12.2, 12.1, 12.4, 12.3, 12.6], dtype=np.float64)
///
/// open2 = np.array([20.0, 20.2, 20.5, 20.3, 20.8, 21.0, 20.9, 21.2, 21.1, 21.4,
///                   21.3, 21.6, 21.5, 21.8, 21.7, 22.0, 21.9, 22.2, 22.1, 22.4], dtype=np.float64)
/// close2 = np.array([20.3, 20.6, 20.8, 20.7, 21.0, 21.2, 21.1, 21.4, 21.3, 21.6,
///                    21.5, 21.8, 21.7, 22.0, 21.9, 22.2, 22.1, 22.4, 22.3, 22.6], dtype=np.float64)
///
/// open3 = np.array([30.0, 30.2, 30.5, 30.3, 30.8, 31.0, 30.9, 31.2, 31.1, 31.4,
///                   31.3, 31.6, 31.5, 31.8, 31.7, 32.0, 31.9, 32.2, 32.1, 32.4], dtype=np.float64)
/// close3 = np.array([30.3, 30.6, 30.8, 30.7, 31.0, 31.2, 31.1, 31.4, 31.3, 31.6,
///                    31.5, 31.8, 31.7, 32.0, 31.9, 32.2, 32.1, 32.4, 32.3, 32.6], dtype=np.float64)
///
/// open4 = np.array([40.0, 40.2, 40.5, 40.3, 40.8, 41.0, 40.9, 41.2, 41.1, 41.4,
///                   41.3, 41.6, 41.5, 41.8, 41.7, 42.0, 41.9, 42.2, 42.1, 42.4], dtype=np.float64)
/// close4 = np.array([40.3, 40.6, 40.8, 40.7, 41.0, 41.2, 41.1, 41.4, 41.3, 41.6,
///                    41.5, 41.8, 41.7, 42.0, 41.9, 42.2, 42.1, 42.4, 42.3, 42.6], dtype=np.float64)
///
/// # Prepare inputs for SIMD processing (must be exactly 2, 4, 8, or 16 assets)
/// inputs = [
///     [open1, close1],  # Asset 1
///     [open2, close2],  # Asset 2
///     [open3, close3],  # Asset 3
///     [open4, close4],  # Asset 4
/// ]
///
/// # QSTICK options: [period]
/// options = [14]  # 14-period QSTICK
///
/// # Calculate QSTICK for all assets using SIMD
/// outputs, states = ti.indicators.qstick_simd_by_assets(inputs, options, None)
///
/// # outputs[0] contains QSTICK values for asset 1
/// # outputs[1] contains QSTICK values for asset 2
/// # outputs[2] contains QSTICK values for asset 3
/// # outputs[3] contains QSTICK values for asset 4
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
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<QstickState>)> {
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
        if asset_inputs.len() != rust_qstick::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_qstick::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }

    if options.len() != rust_qstick::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_qstick::OPTIONS_WIDTH,
            options.len()
        )));
    }

    // Convert Python arrays to Rust slices for each asset
    let mut asset_input_arrays: Vec<[&[f64]; rust_qstick::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; rust_qstick::INPUTS_WIDTH] = [
            asset_inputs[0].as_slice()?, // open
            asset_inputs[1].as_slice()?, // close
        ];
        asset_input_arrays.push(input_array);
    }

    // Create array of references for the by_assets function
    let input_refs: Vec<&[&[f64]; rust_qstick::INPUTS_WIDTH]> = asset_input_arrays.iter().collect();

    let options_array: [f64; rust_qstick::OPTIONS_WIDTH] = [options[0]];

    // Call the SIMD by assets function with proper const generic
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_qstick::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_qstick::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_qstick::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_qstick::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_qstick::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_qstick::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_qstick::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_qstick::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let qstick_states: Vec<QstickState> = states
                .into_iter()
                .map(|state| QstickState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                qstick_states,
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}
