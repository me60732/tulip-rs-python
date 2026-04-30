use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::utils::info_to_hashmap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::bbands as bbands_impl;

/// Bollinger Bands State wrapper for Python
#[pyclass]
pub struct BbandsState {
    inner: bbands_impl::IndicatorState,
}

#[pymethods]
impl BbandsState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "Bollinger Bands State - internal state for Bollinger Bands".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for BBANDS: just one array of real values)
    ///
    /// Returns:
    ///     List of output arrays (for BBANDS: [lower_band, middle_band, upper_band])
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != bbands_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "BBANDS requires {} input arrays, got {}",
                bbands_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (BBANDS only takes 1 input)
        let inputs_array: [&[f64]; bbands_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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
            let inner: bbands_impl::IndicatorState =
                serde_json::from_str(&json_str).map_err(|e| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Deserialization error: {}",
                        e
                    ))
                })?;

            self.inner = inner;
            Ok(())
        })
    }

    fn __repr__(&self) -> String {
        "BbandsState(internal)".to_string()
    }
}

/// Bollinger Bands - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 2], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for BBANDS: just one array of real values)
///     options: Array of options (for BBANDS: [period, std_dev])
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (BBANDS has 3 outputs: [lower_band, middle_band, upper_band])
///     - state: BbandsState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> real = np.array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], dtype=np.float64)
///     >>> inputs = [real]  # BBANDS takes 1 input array
///     >>> options = [20.0, 2.0]  # period=20, std_dev=2
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(len(outputs))  # 3 outputs: lower_band, middle_band, upper_band
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, BbandsState)> {
    // Validate inputs count
    if inputs.len() != bbands_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "BBANDS requires {} input arrays, got {}",
            bbands_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "BBANDS requires exactly 2 options (period, std_dev)",
        ));
    }

    // Validate options
    if options[0] < 1.0 || options[1] <= 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Invalid options: period >= 1, std_dev > 0",
        ));
    }

    // Direct extraction for single input (BBANDS only takes 1 input)
    let inputs_array: [&[f64]; bbands_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    // Convert options to fixed-size array
    let options_array: [f64; 2] = [options[0], options[1]];

    match bbands_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = BbandsState { inner: state };
            Ok((outputs, py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "BBANDS calculation error: {}",
            e
        ))),
    }
}

/// Get BBANDS info
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = bbands_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "BBANDS requires exactly 2 options (period, std_dev)",
        ));
    }
    Ok(bbands_impl::min_data(&options))
}

/// Get expected output length
#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "BBANDS requires exactly 2 options (period, std_dev)",
        ));
    }
    Ok(bbands_impl::output_length(data_length, &options))
}

/// Get minimum data required for accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "BBANDS requires exactly 2 options (period, std_dev)",
        ));
    }
    Ok(bbands_impl::min_data_accuracy(&options, decimals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use numpy::{PyArray1, PyArrayMethods};
    use pyo3::Python;

    #[test]
    fn test_bbands_basic() {
        Python::with_gil(|py| {
            // Need enough data for Bollinger Bands calculation
            let data: Vec<f64> = (1..=30).map(|x| x as f64).collect();
            let py_array = PyArray1::from_vec_bound(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![20.0, 2.0]; // Standard Bollinger Bands: 20 period, 2 std dev

            let (outputs, _state) = indicator(inputs, options, None).unwrap();
            assert_eq!(outputs.len(), 3); // BBANDS has 3 outputs: lower, middle, upper
            assert!(outputs[0].len() > 0); // Lower band
            assert!(outputs[1].len() > 0); // Middle band (SMA)
            assert!(outputs[2].len() > 0); // Upper band
        });
    }

    #[test]
    fn test_bbands_batch_indicator() {
        Python::with_gil(|py| {
            // Initial calculation
            let data: Vec<f64> = (1..=25).map(|x| x as f64).collect();
            let py_array = PyArray1::from_vec_bound(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![20.0, 2.0];

            let (outputs, mut state) = indicator(inputs, options, None).unwrap();
            let _ = outputs[0].len();

            // Continue with new data
            let new_data = vec![26.0, 27.0, 28.0];
            let new_py_array = PyArray1::from_vec_bound(py, new_data);
            let new_readonly = new_py_array.readonly();

            let new_inputs = vec![new_readonly];
            let continued_outputs = state.batch_indicator(new_inputs, None).unwrap();

            assert_eq!(continued_outputs.len(), 3); // Still 3 outputs
            assert_eq!(continued_outputs[0].len(), 3); // 3 new values for each output
            assert_eq!(continued_outputs[1].len(), 3);
            assert_eq!(continued_outputs[2].len(), 3);
        });
    }

    #[test]
    fn test_bbands_validation() {
        Python::with_gil(|py| {
            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let py_array = PyArray1::from_vec_bound(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];

            // Test invalid options count
            let result = indicator(inputs.clone(), vec![20.0], None);
            assert!(result.is_err());

            // Test invalid period
            let result = indicator(inputs.clone(), vec![0.0, 2.0], None);
            assert!(result.is_err());

            // Test invalid std_dev
            let result = indicator(inputs.clone(), vec![20.0, 0.0], None);
            assert!(result.is_err());
        });
    }

    #[test]
    fn test_bbands_band_relationship() {
        Python::with_gil(|py| {
            // Test that upper band > middle band > lower band
            let data: Vec<f64> = (1..=30).map(|x| x as f64).collect();
            let py_array = PyArray1::from_vec_bound(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![10.0, 1.0];

            let (outputs, _state) = indicator(inputs, options, None).unwrap();

            let lower_band = &outputs[0];
            let middle_band = &outputs[1];
            let upper_band = &outputs[2];

            // Check that the bands maintain proper order
            for i in 0..lower_band.len() {
                assert!(
                    lower_band[i] <= middle_band[i],
                    "Lower band should be <= middle band at index {}: {} > {}",
                    i,
                    lower_band[i],
                    middle_band[i]
                );
                assert!(
                    middle_band[i] <= upper_band[i],
                    "Middle band should be <= upper band at index {}: {} > {}",
                    i,
                    middle_band[i],
                    upper_band[i]
                );
            }
        });
    }
}
