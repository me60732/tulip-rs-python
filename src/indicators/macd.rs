use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::utils::info_to_hashmap;
use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::macd as macd_impl;

/// MACD State wrapper for Python
#[pyclass]
pub struct MacdState {
    inner: macd_impl::IndicatorState,
}

#[pymethods]
impl MacdState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "MACD State - internal state for Moving Average Convergence Divergence".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for MACD: just one array of real values)
    ///
    /// Returns:
    ///     List of output arrays (for MACD: macd_line, signal_line, histogram)
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != macd_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "MACD requires {} input arrays, got {}",
                macd_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (MACD only takes 1 input)
        let inputs_array: [&[f64]; macd_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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
        "MacdState(internal)".to_string()
    }
}

/// Moving Average Convergence Divergence - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 3], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for MACD: just one array of real values)
///     options: Array of options (for MACD: [short_period, long_period, signal_period])
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (MACD has 3 outputs: [macd_line, signal_line, histogram])
///     - state: MacdState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> real = np.array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], dtype=np.float64)
///     >>> inputs = [real]  # MACD takes 1 input array
///     >>> options = [12.0, 26.0, 9.0]  # short_period=12, long_period=26, signal_period=9
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(len(outputs))  # 3 outputs: macd_line, signal_line, histogram
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, MacdState)> {
    // Validate inputs count
    if inputs.len() != macd_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "MACD requires {} input arrays, got {}",
            macd_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "MACD requires exactly 3 options (short_period, long_period, signal_period)",
        ));
    }

    // Validate periods
    if options[0] < 1.0 || options[1] <= options[0] || options[2] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Invalid periods: short_period >= 1, long_period > short_period, signal_period >= 1",
        ));
    }

    // Direct extraction for single input (MACD only takes 1 input)
    let inputs_array: [&[f64]; macd_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

    // Convert options to fixed-size array
    let options_array: [f64; 3] = [options[0], options[1], options[2]];

    match macd_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = MacdState { inner: state };
            Ok((outputs, py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "MACD calculation error: {}",
            e
        ))),
    }
}

/// Get MACD info
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = macd_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "MACD requires exactly 3 options (short_period, long_period, signal_period)",
        ));
    }
    Ok(macd_impl::min_data(&options))
}

/// Get expected output length - returns tuple of (macd_len, signal_len, histogram_len)
#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<(usize, usize, usize)> {
    if options.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "MACD requires exactly 3 options (short_period, long_period, signal_period)",
        ));
    }
    Ok(macd_impl::output_length(data_length, &options))
}

/// Get minimum data required for accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "MACD requires exactly 3 options (short_period, long_period, signal_period)",
        ));
    }
    Ok(macd_impl::min_data_accuracy(&options, decimals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use numpy::{PyArray1, PyArrayMethods};
    use pyo3::Python;

    #[cfg(test)] // #[test]
    fn test_macd_basic() {
        
        Python::with_gil(|py| {
            // Need enough data for MACD calculation (long_period + signal_period)
            let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
            let py_array = PyArray1::from_vec(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![12.0, 26.0, 9.0]; // Standard MACD periods

            let (outputs, _state) = indicator(inputs, options, None).unwrap();
            assert_eq!(outputs.len(), 3); // MACD has 3 outputs: macd_line, signal_line, histogram
            assert!(outputs[0].len() > 0); // Should have some output
            assert!(outputs[1].len() > 0); // Signal line
            assert!(outputs[2].len() > 0); // Histogram
        });
    }

    #[cfg(test)] // #[test]
    fn test_macd_batch_indicator() {
        
        Python::with_gil(|py| {
            // Initial calculation
            let data: Vec<f64> = (1..=40).map(|x| x as f64).collect();
            let py_array = PyArray1::from_vec(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];
            let options = vec![12.0, 26.0, 9.0];

            let (_, mut state) = indicator(inputs, options, None).unwrap();
            //let initial_len = outputs[0].len();

            // Continue with new data
            let new_data = vec![41.0, 42.0, 43.0];
            let new_py_array = PyArray1::from_vec(py, new_data);
            let new_readonly = new_py_array.readonly();

            let new_inputs = vec![new_readonly];
            let continued_outputs = state.batch_indicator(new_inputs, None).unwrap();

            assert_eq!(continued_outputs.len(), 3); // Still 3 outputs
            assert_eq!(continued_outputs[0].len(), 3); // 3 new values for each output
            assert_eq!(continued_outputs[1].len(), 3);
            assert_eq!(continued_outputs[2].len(), 3);
        });
    }

    #[cfg(test)] // #[test]
    fn test_macd_validation() {
        
        Python::with_gil(|py| {
            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let py_array = PyArray1::from_vec(py, data);
            let readonly = py_array.readonly();

            let inputs = vec![readonly];

            // Test invalid options count
            let result = indicator(inputs.clone(), vec![12.0, 26.0], None);
            assert!(result.is_err());

            // Test invalid period relationships
            let result = indicator(inputs.clone(), vec![26.0, 12.0, 9.0], None); // short > long
            assert!(result.is_err());

            // Test zero period
            let result = indicator(inputs.clone(), vec![0.0, 26.0, 9.0], None);
            assert!(result.is_err());
        });
    }
}
