use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::pivotpoint as rust_pivotpoint;

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct PivotpointState {
    inner: rust_pivotpoint::IndicatorState,
}

#[pymethods]
impl PivotpointState {
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != rust_pivotpoint::INPUTS_WIDTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Expected {} inputs, got {}",
                rust_pivotpoint::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        let input_arrays: [&[f64]; rust_pivotpoint::INPUTS_WIDTH] = [
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
) -> PyResult<(Vec<Vec<f64>>, PivotpointState)> {
    if inputs.len() != rust_pivotpoint::INPUTS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} inputs, got {}",
            rust_pivotpoint::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    if options.len() != rust_pivotpoint::OPTIONS_WIDTH {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Expected {} options, got {}",
            rust_pivotpoint::OPTIONS_WIDTH,
            options.len()
        )));
    }

    let input_arrays: [&[f64]; rust_pivotpoint::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];

    let options_array: [f64; rust_pivotpoint::OPTIONS_WIDTH] = [options[0]];

    match rust_pivotpoint::indicator(&input_arrays, &options_array, optional_outputs.as_deref()) {
        Ok((result, state)) => Ok((result, PivotpointState { inner: state })),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Indicator calculation failed: {:?}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, rust_pivotpoint::INFO)
}

#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    Ok(rust_pivotpoint::min_data(&options))
}

#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    Ok(rust_pivotpoint::min_data_accuracy(&options, decimals))
}

#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    Ok(rust_pivotpoint::output_length(data_len, &options))
}

pub fn register_pivotpoint_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "pivotpoint")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;

    submodule.add_class::<PivotpointState>()?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}
