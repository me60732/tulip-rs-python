use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::elderray as rust_elderray;

/// Elder-ray State wrapper for Python
#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct ElderRayState {
    inner: rust_elderray::IndicatorState,
}

#[pymethods]
impl ElderRayState {
    fn get_info(&self) -> String {
        "Elder-ray State - internal state for Elder-ray".to_string()
    }

    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_elderray::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "ElderRay requires {} input arrays, got {}",
                rust_elderray::INPUTS_WIDTH,
                inputs.len()
            )));
        }
        let inputs_array: [&[f64]; rust_elderray::INPUTS_WIDTH] = [
            inputs[0].as_slice()?,
            inputs[1].as_slice()?,
            inputs[2].as_slice()?,
        ];
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

    fn __getstate__(&self) -> PyResult<HashMap<String, String>> {
        let serialized = serde_json::to_string(&self.inner).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Serialization error: {}", e))
        })?;
        let mut state = HashMap::new();
        state.insert("inner".to_string(), serialized);
        Ok(state)
    }

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
        "ElderRayState(internal)".to_string()
    }
}

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, ElderRayState)> {
    if inputs.len() != rust_elderray::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "ElderRay requires {} input arrays, got {}",
            rust_elderray::INPUTS_WIDTH,
            inputs.len()
        )));
    }
    if options.len() != rust_elderray::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_elderray::OPTIONS_WIDTH,
            options.len()
        )));
    }
    if options[0] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Invalid options: period >= 1",
        ));
    }
    let inputs_array: [&[f64]; rust_elderray::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];
    let options_array: [f64; rust_elderray::OPTIONS_WIDTH] = [options[0]];
    match rust_elderray::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => Ok((outputs, ElderRayState { inner: state })),
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "ElderRay calculation error: {}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_elderray::INFO)
}

#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_elderray::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_elderray::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_elderray::min_data(&options))
}

#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != rust_elderray::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_elderray::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_elderray::output_length(data_length, &options))
}

#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != rust_elderray::OPTIONS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected {} options, got {}",
            rust_elderray::OPTIONS_WIDTH,
            options.len()
        )));
    }
    Ok(rust_elderray::min_data_accuracy(&options, decimals))
}

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_assets(
    inputs: Vec<Vec<PyReadonlyArray1<f64>>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<ElderRayState>)> {
    if inputs.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "No assets provided",
        ));
    }
    let num_assets = inputs.len();
    if !matches!(num_assets, 2 | 4 | 8 | 16) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "SIMD by assets only supports 2, 4, 8, or 16 assets. Got {}",
            num_assets
        )));
    }
    for (asset_idx, asset_inputs) in inputs.iter().enumerate() {
        if asset_inputs.len() != rust_elderray::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Asset {} expected {} inputs, got {}",
                asset_idx,
                rust_elderray::INPUTS_WIDTH,
                asset_inputs.len()
            )));
        }
    }
    if options.len() != rust_elderray::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_elderray::OPTIONS_WIDTH,
            options.len()
        )));
    }
    let mut asset_input_arrays: Vec<[&[f64]; rust_elderray::INPUTS_WIDTH]> =
        Vec::with_capacity(num_assets);
    for asset_inputs in &inputs {
        let input_array: Result<[&[f64]; rust_elderray::INPUTS_WIDTH], _> = asset_inputs
            .iter()
            .map(|arr| arr.as_slice())
            .collect::<Result<Vec<_>, _>>()?
            .try_into();
        match input_array {
            Ok(arr) => asset_input_arrays.push(arr),
            Err(_) => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Failed to convert input arrays",
                ))
            }
        }
    }
    let input_refs: Vec<&[&[f64]; rust_elderray::INPUTS_WIDTH]> =
        asset_input_arrays.iter().collect();
    let options_array: Result<[f64; rust_elderray::OPTIONS_WIDTH], _> = options.try_into();
    let options_array = options_array.map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to convert options to array of length {}",
            rust_elderray::OPTIONS_WIDTH
        ))
    })?;
    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; rust_elderray::INPUTS_WIDTH]; 2] =
                input_refs.as_slice().try_into().unwrap();
            rust_elderray::by_assets::indicator::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; rust_elderray::INPUTS_WIDTH]; 4] =
                input_refs.as_slice().try_into().unwrap();
            rust_elderray::by_assets::indicator::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; rust_elderray::INPUTS_WIDTH]; 8] =
                input_refs.as_slice().try_into().unwrap();
            rust_elderray::by_assets::indicator::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; rust_elderray::INPUTS_WIDTH]; 16] =
                input_refs.as_slice().try_into().unwrap();
            rust_elderray::by_assets::indicator::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };
    match result {
        Ok((results, states)) => {
            let er_states: Vec<ElderRayState> = states
                .into_iter()
                .map(|state| ElderRayState { inner: state })
                .collect();
            Ok((results, er_states))
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
) -> PyResult<(Vec<Vec<Vec<f64>>>, Vec<ElderRayState>)> {
    if options.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "No options provided",
        ));
    }
    let num_options = options.len();
    if !matches!(num_options, 2 | 4 | 8 | 16) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "SIMD by options only supports 2, 4, 8, or 16 options. Got {}",
            num_options
        )));
    }
    if inputs.len() != rust_elderray::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_elderray::INPUTS_WIDTH,
            inputs.len()
        )));
    }
    for (opt_idx, opt) in options.iter().enumerate() {
        if opt.len() != rust_elderray::OPTIONS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Option set {} expected {} values, got {}",
                opt_idx,
                rust_elderray::OPTIONS_WIDTH,
                opt.len()
            )));
        }
    }
    let input_arrays: [&[f64]; rust_elderray::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];
    let mut option_arrays: Vec<[f64; rust_elderray::OPTIONS_WIDTH]> =
        Vec::with_capacity(num_options);
    for opt in &options {
        option_arrays.push([opt[0]]);
    }
    let option_refs: Vec<&[f64; rust_elderray::OPTIONS_WIDTH]> = option_arrays.iter().collect();
    let result = match num_options {
        2 => {
            let opt_array: &[&[f64; rust_elderray::OPTIONS_WIDTH]; 2] =
                option_refs.as_slice().try_into().unwrap();
            rust_elderray::by_options::indicator::<2>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let opt_array: &[&[f64; rust_elderray::OPTIONS_WIDTH]; 4] =
                option_refs.as_slice().try_into().unwrap();
            rust_elderray::by_options::indicator::<4>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let opt_array: &[&[f64; rust_elderray::OPTIONS_WIDTH]; 8] =
                option_refs.as_slice().try_into().unwrap();
            rust_elderray::by_options::indicator::<8>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let opt_array: &[&[f64; rust_elderray::OPTIONS_WIDTH]; 16] =
                option_refs.as_slice().try_into().unwrap();
            rust_elderray::by_options::indicator::<16>(
                &input_arrays,
                opt_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };
    match result {
        Ok((results, states)) => {
            let er_states: Vec<ElderRayState> = states
                .into_iter()
                .map(|state| ElderRayState { inner: state })
                .collect();
            Ok((results, er_states))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by options calculation failed: {:?}",
            e
        ))),
    }
}

pub fn register_elderray_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "elderray")?;
    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_options, &submodule)?)?;
    submodule.add_class::<ElderRayState>()?;
    parent_module.add_submodule(&submodule)?;
    Ok(())
}
