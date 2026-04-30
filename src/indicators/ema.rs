use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::utils::info_to_hashmap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::ema as ema_impl;

/// EMA State wrapper for Python
#[pyclass]
pub struct EmaState {
    inner: ema_impl::IndicatorState,
}

#[pymethods]
impl EmaState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "EMA State - internal state for Exponential Moving Average".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for EMA: just one array of real values)
    ///
    /// Returns:
    ///     List of output arrays (for EMA: just one array)
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != ema_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "EMA requires {} input arrays, got {}",
                ema_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (EMA only takes 1 input)
        let inputs_array: [&[f64]; ema_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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

    /// Implement Python's pickle protocol - returns state as Python dict/primitives
    fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        // Serialize to JSON first, then parse to Python dict
        let json_str = serde_json::to_string(&self.inner).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Serialization error: {}", e))
        })?;

        // Parse JSON string to Python object
        let json_module = py.import("json")?;
        let loads_fn = json_module.getattr("loads")?;
        let py_obj = loads_fn.call1((json_str,))?;

        Ok(py_obj.into())
    }

    /// Implement Python's pickle protocol - restores state from Python dict/primitives
    fn __setstate__(&mut self, state: PyObject) -> PyResult<()> {
        Python::with_gil(|py| {
            // Convert Python object to JSON string
            let json_module = py.import("json")?;
            let dumps_fn = json_module.getattr("dumps")?;
            let json_str: String = dumps_fn.call1((state,))?.extract()?;

            // Deserialize from JSON
            let inner: ema_impl::IndicatorState = serde_json::from_str(&json_str).map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Deserialization error: {}", e))
            })?;

            self.inner = inner;
            Ok(())
        })
    }

    fn __repr__(&self) -> String {
        "EmaState(internal)".to_string()
    }
}

/// Exponential Moving Average - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 1], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for EMA: just one array of real values)
///     options: Array of options (for EMA: just the period)
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (EMA has 1 output array)
///     - state: EmaState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> real = np.array([1, 2, 3, 4, 5], dtype=np.float64)
///     >>> inputs = [real]  # EMA takes 1 input array
///     >>> options = [3.0]  # period = 3
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(outputs[0])  # EMA values
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, EmaState)> {
    // Validate inputs count
    if inputs.len() != ema_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "EMA requires {} input arrays, got {}",
            ema_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "EMA requires exactly 1 option (period)",
        ));
    }

    // Validate period
    if options[0] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Period must be >= 1",
        ));
    }

    // Direct extraction for single input (EMA only takes 1 input)
    let inputs_array: [&[f64]; ema_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    // Convert options to fixed-size array
    let options_array: [f64; 1] = [options[0]];

    match ema_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = EmaState { inner: state };
            Ok((outputs, py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "EMA calculation error: {}",
            e
        ))),
    }
}

/// Get EMA info
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = ema_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "EMA requires exactly 1 option (period)",
        ));
    }
    Ok(ema_impl::min_data(&options))
}

/// Get expected output length
#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "EMA requires exactly 1 option (period)",
        ));
    }
    Ok(ema_impl::output_length(data_length, &options))
}

/// Get minimum data required for accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "EMA requires exactly 1 option (period)",
        ));
    }
    Ok(ema_impl::min_data_accuracy(&options, decimals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use numpy::{PyArray1, PyArrayMethods};
    use pyo3::Python;

    #[test]
    fn test_ema_basic() {
        Python::with_gil(|py| {
            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let py_array = PyArray1::from_vec_bound(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![3.0];

            let (outputs, _state) = indicator(inputs, options, None).unwrap();
            assert_eq!(outputs.len(), 1); // EMA has 1 output
            assert!(outputs[0].len() > 0); // Should have some output

            // EMA starts from the first value, so length should equal input length
            assert_eq!(outputs[0].len(), 5);
        });
    }

    #[test]
    fn test_ema_batch_indicator() {
        Python::with_gil(|py| {
            // Initial calculation
            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let py_array = PyArray1::from_vec_bound(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![3.0];

            let (outputs, mut state) = indicator(inputs, options, None).unwrap();
            assert_eq!(outputs[0].len(), 5);

            // Continue with new data
            let new_data = vec![6.0, 7.0];
            let new_py_array = PyArray1::from_vec_bound(py, new_data);
            let new_readonly = new_py_array.readonly();

            let new_inputs = vec![new_readonly];
            let continued_outputs = state.batch_indicator(new_inputs, None).unwrap();

            assert_eq!(continued_outputs.len(), 1);
            assert_eq!(continued_outputs[0].len(), 2); // 2 new values
        });
    }
}
