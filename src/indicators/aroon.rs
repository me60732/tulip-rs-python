use crate::utils::info_to_hashmap;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::aroon as aroon_impl;

/// AROON State wrapper for Python
#[pyclass]
pub struct AroonState {
    inner: aroon_impl::IndicatorState,
}

#[pymethods]
impl AroonState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "AROON State - internal state for Aroon Trend Indicator".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays [high, low]
    ///
    /// Returns:
    ///     List of output arrays [aroon_down, aroon_up]
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != aroon_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "AROON requires {} input arrays, got {}",
                aroon_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for two inputs (high, low)
        let inputs_array: [&[f64]; aroon_impl::INPUTS_WIDTH] =
            [inputs[0].as_slice()?, inputs[1].as_slice()?];

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
}

/// Calculate AROON (Aroon Trend Indicator)
///
/// The Aroon indicator is used to identify trend changes and the strength of a trend.
/// It consists of Aroon Up and Aroon Down lines that oscillate between 0 and 100.
///
/// Parameters:
/// - inputs: List of numpy arrays [high, low]
/// - options: List containing [period]
/// - optional_outputs: Optional list of booleans for additional outputs (none available)
///
/// Returns:
/// - Tuple of (outputs, state) where outputs is [aroon_down, aroon_up]
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, AroonState)> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "AROON requires exactly 1 option: period",
        ));
    }

    if inputs.len() != aroon_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "AROON requires {} input arrays, got {}",
            aroon_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Direct extraction for two inputs (high, low)
    let inputs_array: [&[f64]; aroon_impl::INPUTS_WIDTH] =
        [inputs[0].as_slice()?, inputs[1].as_slice()?];
    let options_array: [f64; 1] = [options[0]];

    match aroon_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => Ok((outputs, AroonState { inner: state })),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Calculation error: {}",
            e
        ))),
    }
}

/// Get AROON indicator information
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = aroon_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data length required for AROON calculation
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "AROON requires exactly 1 option: period",
        ));
    }
    Ok(aroon_impl::min_data(&options))
}

/// Get minimum data length required for AROON calculation with accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "AROON requires exactly 1 option: period",
        ));
    }
    Ok(aroon_impl::min_data_accuracy(&options, decimals))
}

/// Get output length for AROON calculation
#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "AROON requires exactly 1 option: period",
        ));
    }
    Ok(aroon_impl::output_length(data_len, &options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tulip_rs::indicator_types::TIndicatorState;

    #[cfg(test)] // #[test]
    fn test_aroon_basic() {
        let high = [
            82.15, 81.89, 83.03, 83.30, 83.85, 83.90, 83.33, 84.30, 84.84, 85.00, 85.90, 86.58,
            86.98, 88.00, 87.87,
        ];
        let low = [
            81.29, 80.64, 81.31, 82.65, 83.07, 83.11, 82.49, 82.30, 84.15, 84.11, 84.03, 85.39,
            85.76, 87.17, 87.01,
        ];
        let options = vec![5.0];

        let input_refs: [&[f64]; 2] = [&high, &low];
        let options_array: [f64; 1] = [options[0]];

        let result = aroon_impl::indicator(&input_refs, &options_array, None);
        assert!(result.is_ok());
        let (outputs, _state) = result.unwrap();
        assert!(!outputs[0].is_empty());
        assert!(!outputs[1].is_empty());
    }

    #[cfg(test)] // #[test]
    fn test_aroon_state_continuation() {
        let high = [
            82.15, 81.89, 83.03, 83.30, 83.85, 83.90, 83.33, 84.30, 84.84, 85.00, 85.90, 86.58,
            86.98, 88.00, 87.87,
        ];
        let low = [
            81.29, 80.64, 81.31, 82.65, 83.07, 83.11, 82.49, 82.30, 84.15, 84.11, 84.03, 85.39,
            85.76, 87.17, 87.01,
        ];
        let options = vec![5.0];

        // Test state continuation
        let split_point = 10;
        let input_refs1: [&[f64]; 2] = [&high[..split_point], &low[..split_point]];
        let input_refs2: [&[f64]; 2] = [&high[split_point..], &low[split_point..]];
        let options_array: [f64; 1] = [options[0]];

        let (_outputs1, mut state) =
            aroon_impl::indicator(&input_refs1, &options_array, None).unwrap();
        let outputs2 = state.batch_indicator(&input_refs2, None).unwrap();

        assert!(!outputs2[0].is_empty());
        assert!(!outputs2[1].is_empty());
    }
}
