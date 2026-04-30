use crate::utils::info_to_hashmap;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::stoch as stoch_impl;

/// STOCH State wrapper for Python
#[pyclass]
pub struct StochState {
    inner: stoch_impl::IndicatorState,
}

#[pymethods]
impl StochState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "STOCH State - internal state for Stochastic Oscillator".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays (for STOCH: [high, low, close] arrays)
    ///
    /// Returns:
    ///     List of output arrays (for STOCH: [%K, %D])
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != stoch_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "STOCH requires {} input arrays, got {}",
                stoch_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for three inputs (STOCH takes high, low, close)
        let inputs_array: [&[f64]; stoch_impl::INPUTS_WIDTH] = [
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
        "StochState(internal)".to_string()
    }
}

/// Stochastic Oscillator - returns (outputs, state) tuple just like Rust
///
/// Mirrors the Rust signature: indicator(inputs: &[&[f64]; INPUTS_WIDTH], options: &[f64; 3], optional_outputs: Option<&[bool]>)
///
/// Args:
///     inputs: Array of input arrays (for STOCH: [high, low, close] arrays)
///     options: Array of options (for STOCH: [k_period, k_slow_period, d_period])
///     optional_outputs: Optional array of booleans for selecting outputs (None for all)
///
/// Returns:
///     Tuple of (outputs, state) where:
///     - outputs: List of arrays (STOCH has 2 outputs: [%K, %D])
///     - state: StochState for continuing calculations
///
/// Example:
///     >>> import numpy as np
///     >>> high = np.array([82, 83, 84, 85, 86], dtype=np.float64)
///     >>> low = np.array([80, 81, 82, 83, 84], dtype=np.float64)
///     >>> close = np.array([81, 82, 83, 84, 85], dtype=np.float64)
///     >>> inputs = [high, low, close]  # STOCH takes 3 input arrays
///     >>> options = [14.0, 3.0, 3.0]  # k_period=14, k_slow_period=3, d_period=3
///     >>> outputs, state = indicator(inputs, options, None)
///     >>> print(outputs[0])  # %K values
///     >>> print(outputs[1])  # %D values
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, StochState)> {
    // Validate inputs count
    if inputs.len() != stoch_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "STOCH requires {} input arrays, got {}",
            stoch_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Validate options count
    if options.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "STOCH requires exactly 3 options (k_period, k_slow_period, d_period)",
        ));
    }

    // Validate periods
    if options[0] < 1.0 || options[1] < 1.0 || options[2] < 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "All periods must be >= 1",
        ));
    }

    // Direct extraction for three inputs (STOCH takes high, low, close)
    let inputs_array: [&[f64]; stoch_impl::INPUTS_WIDTH] = [
        inputs[0].as_slice()?,
        inputs[1].as_slice()?,
        inputs[2].as_slice()?,
    ];

    // Convert options to fixed-size array
    let options_array: [f64; 3] = [options[0], options[1], options[2]];

    match stoch_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => {
            let py_state = StochState { inner: state };
            Ok((outputs, py_state))
        }
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "STOCH calculation error: {}",
            e
        ))),
    }
}

/// Get STOCH info
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = stoch_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data required
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "STOCH requires exactly 3 options (k_period, k_slow_period, d_period)",
        ));
    }
    Ok(stoch_impl::min_data(&options))
}

/// Get expected output length - returns tuple of (k_len, d_len)
#[pyfunction]
pub fn output_length(data_length: usize, options: Vec<f64>) -> PyResult<(usize, usize)> {
    if options.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "STOCH requires exactly 3 options (k_period, k_slow_period, d_period)",
        ));
    }
    Ok(stoch_impl::output_length(data_length, &options))
}

/// Get minimum data required for accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "STOCH requires exactly 3 options (k_period, k_slow_period, d_period)",
        ));
    }
    Ok(stoch_impl::min_data_accuracy(&options, decimals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use numpy::{PyArray1, PyArrayMethods};
    use pyo3::Python;

    #[cfg(test)] // #[test]
    fn test_stoch_basic() {
        
        Python::with_gil(|py| {
            // Need enough data for STOCH calculation
            let high: Vec<f64> = (80..100).map(|x| x as f64 + 2.0).collect(); // 82-101
            let low: Vec<f64> = (80..100).map(|x| x as f64).collect(); // 80-99
            let close: Vec<f64> = (80..100).map(|x| x as f64 + 1.0).collect(); // 81-100

            let high_array = PyArray1::from_vec(py, high);
            let low_array = PyArray1::from_vec(py, low);
            let close_array = PyArray1::from_vec(py, close);

            let inputs = vec![
                high_array.readonly(),
                low_array.readonly(),
                close_array.readonly(),
            ];
            let options = vec![14.0, 3.0, 3.0]; // Standard STOCH periods

            let (outputs, _state) = indicator(inputs, options, None).unwrap();
            assert_eq!(outputs.len(), 2); // STOCH has 2 outputs: %K and %D
            assert!(outputs[0].len() > 0); // %K values
            assert!(outputs[1].len() > 0); // %D values
        });
    }

    #[cfg(test)] // #[test]
    fn test_stoch_batch_indicator() {
        
        Python::with_gil(|py| {
            // Initial calculation
            let high: Vec<f64> = (80..95).map(|x| x as f64 + 2.0).collect();
            let low: Vec<f64> = (80..95).map(|x| x as f64).collect();
            let close: Vec<f64> = (80..95).map(|x| x as f64 + 1.0).collect();

            let high_array = PyArray1::from_vec(py, high);
            let low_array = PyArray1::from_vec(py, low);
            let close_array = PyArray1::from_vec(py, close);

            let inputs = vec![
                high_array.readonly(),
                low_array.readonly(),
                close_array.readonly(),
            ];
            let options = vec![10.0, 3.0, 3.0];

            let (outputs, mut state) = indicator(inputs, options, None).unwrap();
            let _ = outputs[0].len();

            // Continue with new data
            let new_high = vec![99.0, 100.0];
            let new_low = vec![97.0, 98.0];
            let new_close = vec![98.0, 99.0];

            let new_high_array = PyArray1::from_vec(py, new_high);
            let new_low_array = PyArray1::from_vec(py, new_low);
            let new_close_array = PyArray1::from_vec(py, new_close);

            let new_inputs = vec![
                new_high_array.readonly(),
                new_low_array.readonly(),
                new_close_array.readonly(),
            ];
            let continued_outputs = state.batch_indicator(new_inputs, None).unwrap();

            assert_eq!(continued_outputs.len(), 2); // Still 2 outputs
            assert_eq!(continued_outputs[0].len(), 2); // 2 new values for each output
            assert_eq!(continued_outputs[1].len(), 2);
        });
    }

    #[cfg(test)] // #[test]
    fn test_stoch_validation() {
        
        Python::with_gil(|py| {
            let high = vec![82.0, 83.0, 84.0];
            let low = vec![80.0, 81.0, 82.0];
            let close = vec![81.0, 82.0, 83.0];

            let high_array = PyArray1::from_vec(py, high);
            let low_array = PyArray1::from_vec(py, low);
            let close_array = PyArray1::from_vec(py, close);

            // Test wrong number of inputs
            let inputs = vec![high_array.readonly(), low_array.readonly()]; // Missing close
            let result = indicator(inputs, vec![14.0, 3.0, 3.0], None);
            assert!(result.is_err());

            // Test wrong number of options
            let inputs = vec![
                high_array.readonly(),
                low_array.readonly(),
                close_array.readonly(),
            ];
            let result = indicator(inputs, vec![14.0, 3.0], None); // Missing d_period
            assert!(result.is_err());

            // Test invalid periods
            let inputs = vec![
                high_array.readonly(),
                low_array.readonly(),
                close_array.readonly(),
            ];
            let result = indicator(inputs, vec![0.0, 3.0, 3.0], None); // Invalid k_period
            assert!(result.is_err());
        });
    }
}
