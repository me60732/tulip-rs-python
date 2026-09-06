use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::pivotpoint::{min_data, IndicatorState, INFO, INPUTS, OPTIONS};

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct PivotpointState {
    inner: IndicatorState,
}

#[pymethods]
impl PivotpointState {
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

    fn __repr__(&self) -> String {
        "PivotpointState(internal)".to_string()
    }
}

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    py: Python<'_>,
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Py<PyArray1<f64>>>, PivotpointState)> {
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

    // Pivotpoint uses a slice for options (not array), but we need to convert
    if options.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Pivotpoint requires at least one option",
        ));
    }

    match tulip_rs::indicators::pivotpoint::indicator(
        &input_arrays,
        &[options[0]],
        optional_outputs.as_deref(),
    ) {
        Ok((result, state)) => Ok((
            crate::utils::vecs_to_pyarrays(py, result),
            PivotpointState { inner: state },
        )),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Indicator calculation failed: {:?}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, INFO)
}

#[pyfunction]
pub fn min_data_func(options: Vec<f64>) -> PyResult<usize> {
    Ok(min_data(&options))
}

pub fn register_pivotpoint_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "pivotpoint")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_func, &submodule)?)?;

    submodule.add_class::<PivotpointState>()?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}
