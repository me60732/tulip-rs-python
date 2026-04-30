use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::utils::info_to_hashmap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::sma as sma_impl;

/// SMA State wrapper for Python
#[pyclass]
pub struct SmaState {
    inner: sma_impl::IndicatorState,
}

#[pymethods]
impl SmaState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "SMA State - internal state for Simple Moving Average".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for SMA: just one array of real values)
    ///
    /// Returns:
    ///     List of output arrays (for SMA: just one array)
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != sma_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "SMA requires {} input arrays, got {}",
                sma_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (SMA only takes 1 input)
        let inputs_array: [&[f64]; sma_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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
    fn __getstate__(&self) -> PyResult<HashMap<String, String>> {
        let serialized = serde_json::to_string(&self.inner).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Serialization error: {}", e))
        })?;
        let mut state = HashMap::new();
        state.insert("inner".to_string(), serialized);
        Ok(state)
    }

    /// Implement Python's pickle protocol - restores state from Python dict/primitives
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
        "SmaState(internal)".to_string()
    }
}

/// Simple Moving Average - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 1], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for SMA: just one array of real values)
///     options: Array of options (for SMA: just the period)
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (SMA has 1 output array)
///     - state: SmaState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> real = np.array([1, 2, 3, 4, 5], dtype=np.float64)
///     >>> inputs = [real]  # SMA takes 1 input array
///     >>> options = [3.0]  # period = 3
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(outputs[0])  # [2.0, 3.0, 4.0]
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, SmaState)> {
    // Validate inputs count
    if inputs.len() != sma_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "SMA requires {} input arrays, got {}",
            sma_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "SMA requires exactly 1 option (period)",
        ));
    }

    // Validate period
    if options[0] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Period must be >= 1",
        ));
    }

    // Direct extraction for single input (SMA only takes 1 input)
    let inputs_array: [&[f64]; sma_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    // Convert options to fixed-size array
    let options_array: [f64; 1] = [options[0]];

    match sma_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = SmaState { inner: state };
            Ok((outputs, py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "SMA calculation error: {}",
            e
        ))),
    }
}

/// Get SMA info
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = sma_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "SMA requires exactly 1 option (period)",
        ));
    }
    let options_array: [f64; 1] = [options[0]];
    Ok(sma_impl::min_data(&options_array))
}

/// Get expected output length
#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "SMA requires exactly 1 option (period)",
        ));
    }
    let options_array: [f64; 1] = [options[0]];
    Ok(sma_impl::output_length(data_length, &options_array))
}

/// Get minimum data required for accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "SMA requires exactly 1 option (period)",
        ));
    }
    let options_array: [f64; 1] = [options[0]];
    Ok(sma_impl::min_data_accuracy(&options_array, decimals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use numpy::{PyArray1, PyArrayMethods};
    use pyo3::Python;

    #[cfg(test)] // #[test]
    fn test_sma_basic() {
        
        Python::with_gil(|py| {
            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let py_array = PyArray1::from_vec(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![3.0];

            let (outputs, _state) = indicator(inputs, options, None).unwrap();
            assert_eq!(outputs.len(), 1); // SMA has 1 output
            assert_eq!(outputs[0].len(), 3); // 5 - 3 + 1 = 3

            // Expected: [2.0, 3.0, 4.0]
            assert!((outputs[0][0] - 2.0).abs() < 1e-10);
            assert!((outputs[0][1] - 3.0).abs() < 1e-10);
            assert!((outputs[0][2] - 4.0).abs() < 1e-10);
        });
    }

    #[cfg(test)] // #[test]
    fn test_sma_batch_indicator() {
        
        Python::with_gil(|py| {
            // Initial calculation
            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let py_array = PyArray1::from_vec(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![3.0];

            let (outputs, mut state) = indicator(inputs, options, None).unwrap();
            assert_eq!(outputs[0].len(), 3);

            // Continue with new data
            let new_data = vec![6.0, 7.0];
            let new_py_array = PyArray1::from_vec(py, new_data);
            let new_readonly = new_py_array.readonly();

            let new_inputs = vec![new_readonly];
            let continued_outputs = state.batch_indicator(new_inputs, None).unwrap();

            assert_eq!(continued_outputs.len(), 1);
            assert_eq!(continued_outputs[0].len(), 2); // 2 new values

            // Expected: [5.0, 6.0] (averages of [4,5,6] and [5,6,7])
            assert!((continued_outputs[0][0] - 5.0).abs() < 1e-10);
            assert!((continued_outputs[0][1] - 6.0).abs() < 1e-10);
        });
    }
}
