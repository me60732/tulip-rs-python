use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::homodynediscriminator::{
    HomodyneDiscriminator, Indicator, IndicatorState, INPUTS, OPTIONS,
};

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct HomodynediscriminatorState {
    inner: IndicatorState,
}

#[pymethods]
impl HomodynediscriminatorState {
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

        let input_arrays: [&[f64]; INPUTS] = [inputs[0].as_slice()?];

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
) -> PyResult<(Vec<Py<PyArray1<f64>>>, HomodynediscriminatorState)> {
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

    let input_arrays: [&[f64]; INPUTS] = [inputs[0].as_slice()?];

    // HomodyneDiscriminator has 0 options
    let options_array: [f64; OPTIONS] = [];

    match HomodyneDiscriminator::indicator(
        &input_arrays,
        &options_array,
        optional_outputs.as_deref(),
    ) {
        Ok((result, state)) => Ok((
            crate::utils::vecs_to_pyarrays(py, result),
            HomodynediscriminatorState { inner: state },
        )),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Indicator calculation failed: {:?}",
            e
        ))),
    }
}

#[pyfunction]
pub fn info(py: Python<'_>) -> PyResult<Bound<'_, pyo3::types::PyDict>> {
    crate::utils::info_to_pydict(py, HomodyneDiscriminator::INFO)
}

#[pyfunction]
pub fn min_data(_options: Vec<f64>) -> PyResult<usize> {
    let options_array: [f64; OPTIONS] = [];
    Ok(HomodyneDiscriminator::min_data(&options_array))
}

#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn simd_by_assets(
    py: Python<'_>,
    inputs: Vec<Vec<PyReadonlyArray1<f64>>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<Py<PyArray1<f64>>>>, Vec<HomodynediscriminatorState>)> {
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

    let mut asset_input_arrays: Vec<[&[f64]; INPUTS]> = Vec::with_capacity(num_assets);

    for asset_inputs in &inputs {
        let input_array: [&[f64]; INPUTS] = [asset_inputs[0].as_slice()?];
        asset_input_arrays.push(input_array);
    }

    let input_refs: Vec<&[&[f64]; INPUTS]> = asset_input_arrays.iter().collect();

    let options_array: [f64; OPTIONS] = [];

    let result = match num_assets {
        2 => {
            let input_array: &[&[&[f64]; INPUTS]; 2] = input_refs.as_slice().try_into().unwrap();
            HomodyneDiscriminator::indicator_by_assets::<2>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        4 => {
            let input_array: &[&[&[f64]; INPUTS]; 4] = input_refs.as_slice().try_into().unwrap();
            HomodyneDiscriminator::indicator_by_assets::<4>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        8 => {
            let input_array: &[&[&[f64]; INPUTS]; 8] = input_refs.as_slice().try_into().unwrap();
            HomodyneDiscriminator::indicator_by_assets::<8>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        16 => {
            let input_array: &[&[&[f64]; INPUTS]; 16] = input_refs.as_slice().try_into().unwrap();
            HomodyneDiscriminator::indicator_by_assets::<16>(
                input_array,
                &options_array,
                optional_outputs.as_deref(),
            )
        }
        _ => unreachable!("Already validated SIMD lane count"),
    };

    match result {
        Ok((results, states)) => {
            let homodynediscriminator_states: Vec<HomodynediscriminatorState> = states
                .into_iter()
                .map(|state| HomodynediscriminatorState { inner: state })
                .collect();
            Ok((
                crate::utils::simd_vecs_to_pyarrays(py, results),
                homodynediscriminator_states,
            ))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "SIMD by assets calculation failed: {:?}",
            e
        ))),
    }
}

pub fn register_homodynediscriminator_module(
    parent_module: &pyo3::Bound<'_, PyModule>,
) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "homodynediscriminator")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;

    submodule.add_function(pyo3::wrap_pyfunction!(simd_by_assets, &submodule)?)?;
    submodule.add_class::<HomodynediscriminatorState>()?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}
