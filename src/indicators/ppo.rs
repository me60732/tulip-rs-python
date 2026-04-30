use crate::utils::info_to_hashmap;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::ppo as ppo_impl;

/// PPO State wrapper for Python
#[pyclass]
pub struct PpoState {
    inner: ppo_impl::IndicatorState,
}

#[pymethods]
impl PpoState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "PPO State - internal state for Percentage Price Oscillator".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays [real]
    ///
    /// Returns:
    ///     List of output arrays [ppo] + optional outputs [short_ema, long_ema]
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != ppo_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "PPO requires {} input arrays, got {}",
                ppo_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (real)
        let inputs_array: [&[f64]; ppo_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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

/// Calculate PPO (Percentage Price Oscillator)
///
/// The Percentage Price Oscillator (PPO) is similar to MACD but measures the
/// percentage difference between two exponential moving averages.
///
/// Parameters:
/// - inputs: List of numpy arrays [real]
/// - options: List containing [short_period, long_period]
/// - optional_outputs: Optional list of booleans for additional outputs [short_ema, long_ema]
///
/// Returns:
/// - Tuple of (outputs, state) where outputs is [ppo_line] + optional outputs
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, PpoState)> {
    if options.len() != 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "PPO requires exactly 2 options: short_period, long_period",
        ));
    }

    if inputs.len() != ppo_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "PPO requires {} input arrays, got {}",
            ppo_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Direct extraction for single input (real)
    let inputs_array: [&[f64]; ppo_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];
    let options_array: [f64; 2] = [options[0], options[1]];

    match ppo_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => Ok((outputs, PpoState { inner: state })),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Calculation error: {}",
            e
        ))),
    }
}

/// Get PPO indicator information
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = ppo_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data length required for PPO calculation
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "PPO requires exactly 2 options: short_period, long_period",
        ));
    }
    Ok(ppo_impl::min_data(&options))
}

/// Get minimum data length required for PPO calculation with accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "PPO requires exactly 2 options: short_period, long_period",
        ));
    }
    Ok(ppo_impl::min_data_accuracy(&options, decimals))
}

/// Get output length for PPO calculation
#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "PPO requires exactly 2 options: short_period, long_period",
        ));
    }
    Ok(ppo_impl::output_length(data_len, &options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tulip_rs::indicator_types::TIndicatorState;

    #[cfg(test)] // #[test]
    fn test_ppo_basic() {
        let close = [
            81.59, 81.06, 82.87, 83.00, 83.61, 83.15, 82.84, 83.99, 84.55, 84.36, 85.53, 86.54,
            86.89, 87.77, 87.29,
        ];
        let options = vec![2.0, 5.0];

        let input_refs: [&[f64]; 1] = [&close];
        let options_array: [f64; 2] = [options[0], options[1]];

        let result = ppo_impl::indicator(&input_refs, &options_array, None);
        assert!(result.is_ok());
        let (outputs, _state) = result.unwrap();
        assert!(!outputs[0].is_empty());
    }

    #[cfg(test)] // #[test]
    fn test_ppo_state_continuation() {
        let close = [
            81.59, 81.06, 82.87, 83.00, 83.61, 83.15, 82.84, 83.99, 84.55, 84.36, 85.53, 86.54,
            86.89, 87.77, 87.29,
        ];
        let options = vec![2.0, 5.0];

        // Test state continuation
        let split_point = 10;
        let input_refs1: [&[f64]; 1] = [&close[..split_point]];
        let input_refs2: [&[f64]; 1] = [&close[split_point..]];
        let options_array: [f64; 2] = [options[0], options[1]];

        let (_outputs1, mut state) =
            ppo_impl::indicator(&input_refs1, &options_array, None).unwrap();
        let outputs2 = state.batch_indicator(&input_refs2, None).unwrap();

        assert!(!outputs2[0].is_empty());
    }
}
