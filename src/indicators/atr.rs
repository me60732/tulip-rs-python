use crate::utils::info_to_hashmap;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::atr as atr_impl;

/// ATR State wrapper for Python
#[pyclass]
pub struct AtrState {
    inner: atr_impl::IndicatorState,
}

#[pymethods]
impl AtrState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "ATR State - internal state for Average True Range".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for ATR: [high, low, close] arrays)
    ///
    /// Returns:
    ///     List of output arrays (for ATR: [atr] or [atr, tr] if optional outputs enabled)
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != atr_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "ATR requires {} input arrays, got {}",
                atr_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for three inputs (ATR takes high, low, close)
        let inputs_array: [&[f64]; atr_impl::INPUTS_WIDTH] = [
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
            let inner: atr_impl::IndicatorState = serde_json::from_str(&json_str).map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Deserialization error: {}", e))
            })?;

            self.inner = inner;
            Ok(())
        })
    }

    fn __repr__(&self) -> String {
        "AtrState(internal)".to_string()
    }
}

/// Average True Range - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 1], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for ATR: [high, low, close] arrays)
///     options: Array of options (for ATR: just the period)
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (ATR has 1-2 outputs: [atr] or [atr, tr])
///     - state: AtrState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> high = np.array([82, 83, 84, 85, 86], dtype=np.float64)
///     >>> low = np.array([80, 81, 82, 83, 84], dtype=np.float64)
///     >>> close = np.array([81, 82, 83, 84, 85], dtype=np.float64)
///     >>> inputs = [high, low, close]  # ATR takes 3 input arrays
///     >>> options = [14.0]  # period = 14
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(outputs[0])  # ATR values
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, AtrState)> {
    // Validate inputs count
    if inputs.len() != atr_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "ATR requires {} input arrays, got {}",
            atr_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "ATR requires exactly 1 option (period)",
        ));
    }

    // Validate period
    if options[0] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Period must be >= 1",
        ));
    }

    // Direct extraction for three inputs (ATR takes high, low, close)
    let inputs_array: [&[f64]; atr_impl::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];

    // Convert options to fixed-size array
    let options_array: [f64; 1] = [options[0]];

    match atr_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = AtrState { inner: state };
            Ok((outputs, py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "ATR calculation error: {}",
            e
        ))),
    }
}

/// Get ATR info
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = atr_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "ATR requires exactly 1 option (period)",
        ));
    }
    Ok(atr_impl::min_data(&options))
}

/// Get expected output length
#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "ATR requires exactly 1 option (period)",
        ));
    }
    Ok(atr_impl::output_length(data_length, &options))
}

/// Get minimum data required for accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "ATR requires exactly 1 option (period)",
        ));
    }
    Ok(atr_impl::min_data_accuracy(&options, decimals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use numpy::{PyArray1, PyArrayMethods};
    use pyo3::Python;

    #[test]
    fn test_atr_basic() {
        Python::with_gil(|py| {
            let high = vec![82.0, 83.0, 84.0, 85.0, 86.0, 87.0, 88.0, 89.0, 90.0, 91.0];
            let low = vec![80.0, 81.0, 82.0, 83.0, 84.0, 85.0, 86.0, 87.0, 88.0, 89.0];
            let close = vec![81.0, 82.0, 83.0, 84.0, 85.0, 86.0, 87.0, 88.0, 89.0, 90.0];

            let high_array = PyArray1::from_vec_bound(py, high);
            let low_array = PyArray1::from_vec_bound(py, low);
            let close_array = PyArray1::from_vec_bound(py, close);

            let inputs = vec![
                high_array.readonly(),
                low_array.readonly(),
                close_array.readonly(),
            ];
            let options = vec![5.0]; // ATR period = 5

            let (outputs, _state) = indicator(inputs, options, None).unwrap();
            assert_eq!(outputs.len(), 1); // ATR has 1 output by default
            assert!(outputs[0].len() > 0); // Should have some output
        });
    }

    #[test]
    fn test_atr_batch_indicator() {
        Python::with_gil(|py| {
            // Initial calculation
            let high = vec![82.0, 83.0, 84.0, 85.0, 86.0, 87.0, 88.0];
            let low = vec![80.0, 81.0, 82.0, 83.0, 84.0, 85.0, 86.0];
            let close = vec![81.0, 82.0, 83.0, 84.0, 85.0, 86.0, 87.0];

            let high_array = PyArray1::from_vec_bound(py, high);
            let low_array = PyArray1::from_vec_bound(py, low);
            let close_array = PyArray1::from_vec_bound(py, close);

            let inputs = vec![
                high_array.readonly(),
                low_array.readonly(),
                close_array.readonly(),
            ];
            let options = vec![5.0];

            let (outputs, mut state) = indicator(inputs, options, None).unwrap();
            let _ = outputs[0].len();

            // Continue with new data
            let new_high = vec![89.0, 90.0];
            let new_low = vec![87.0, 88.0];
            let new_close = vec![88.0, 89.0];

            let new_high_array = PyArray1::from_vec_bound(py, new_high);
            let new_low_array = PyArray1::from_vec_bound(py, new_low);
            let new_close_array = PyArray1::from_vec_bound(py, new_close);

            let new_inputs = vec![
                new_high_array.readonly(),
                new_low_array.readonly(),
                new_close_array.readonly(),
            ];
            let continued_outputs = state.batch_indicator(new_inputs, None).unwrap();

            assert_eq!(continued_outputs.len(), 1);
            assert_eq!(continued_outputs[0].len(), 2); // 2 new values
        });
    }

    #[test]
    fn test_atr_validation() {
        Python::with_gil(|py| {
            let high = vec![82.0, 83.0, 84.0];
            let low = vec![80.0, 81.0, 82.0];
            let close = vec![81.0, 82.0, 83.0];

            let high_array = PyArray1::from_vec_bound(py, high);
            let low_array = PyArray1::from_vec_bound(py, low);
            let close_array = PyArray1::from_vec_bound(py, close);

            // Test wrong number of inputs
            let inputs = vec![high_array.readonly(), low_array.readonly()]; // Missing close
            let result = indicator(inputs, vec![5.0], None);
            assert!(result.is_err());

            // Test wrong number of options
            let inputs = vec![
                high_array.readonly(),
                low_array.readonly(),
                close_array.readonly(),
            ];
            let result = indicator(inputs, vec![5.0, 10.0], None); // Too many options
            assert!(result.is_err());

            // Test invalid period
            let inputs = vec![
                high_array.readonly(),
                low_array.readonly(),
                close_array.readonly(),
            ];
            let result = indicator(inputs, vec![0.0], None); // Invalid period
            assert!(result.is_err());
        });
    }
}
