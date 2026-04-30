use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::utils::info_to_hashmap;

use tulip_rs::indicator_types::TIndicatorState;
use tulip_rs::indicators::stochrsi as stochrsi_impl;

/// STOCHRSI State wrapper for Python
#[pyclass]
pub struct StochrsiState {
    inner: stochrsi_impl::IndicatorState,
}

#[pymethods]
impl StochrsiState {
    /// Get indicator info
    fn get_info(&self) -> String {
        "STOCHRSI State - internal state for Stochastic RSI".to_string()
    }

    /// Continue calculation with new data
    ///
    /// Args:
    ///     inputs: Array of input arrays [real]
    ///
    /// Returns:
    ///     List of output arrays [stochrsi] + optional outputs [rsi]
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        if inputs.len() != stochrsi_impl::INPUTS_WIDTH {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "STOCHRSI requires {} input arrays, got {}",
                stochrsi_impl::INPUTS_WIDTH,
                inputs.len()
            )));
        }

        // Direct extraction for single input (real)
        let inputs_array: [&[f64]; stochrsi_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];

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
        // Serialize the internal state to JSON
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
            let inner: stochrsi_impl::IndicatorState =
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
}

/// Calculate STOCHRSI (Stochastic RSI)
///
/// The Stochastic RSI is a momentum oscillator that combines the Stochastic
/// oscillator formula with RSI values instead of price data.
///
/// Parameters:
/// - inputs: List of numpy arrays [real]
/// - options: List containing [period]
/// - optional_outputs: Optional list of booleans for additional outputs [rsi]
///
/// Returns:
/// - Tuple of (outputs, state) where outputs is [stochrsi_line] + optional outputs
#[pyfunction]
#[pyo3(signature = (inputs, options, optional_outputs=None))]
pub fn indicator(
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, StochrsiState)> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "STOCHRSI requires exactly 1 option: period",
        ));
    }

    if inputs.len() != stochrsi_impl::INPUTS_WIDTH {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "STOCHRSI requires {} input arrays, got {}",
            stochrsi_impl::INPUTS_WIDTH,
            inputs.len()
        )));
    }

    // Direct extraction for single input (real)
    let inputs_array: [&[f64]; stochrsi_impl::INPUTS_WIDTH] = [inputs[0].as_slice()?];
    let options_array: [f64; 1] = [options[0]];

    match stochrsi_impl::indicator(&inputs_array, &options_array, optional_outputs.as_deref()) {
        Ok((outputs, state)) => Ok((outputs, StochrsiState { inner: state })),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Calculation error: {}",
            e
        ))),
    }
}

/// Get STOCHRSI indicator information
#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    let info = stochrsi_impl::info();
    Ok(info_to_hashmap(info))
}

/// Get minimum data length required for STOCHRSI calculation
#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "STOCHRSI requires exactly 1 option: period",
        ));
    }
    Ok(stochrsi_impl::min_data(&options))
}

/// Get minimum data length required for STOCHRSI calculation with accuracy
#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "STOCHRSI requires exactly 1 option: period",
        ));
    }
    Ok(stochrsi_impl::min_data_accuracy(&options, decimals))
}

/// Get output length for STOCHRSI calculation
#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    if options.len() != 1 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "STOCHRSI requires exactly 1 option: period",
        ));
    }
    Ok(stochrsi_impl::output_length(data_len, &options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tulip_rs::indicator_types::TIndicatorState;

    #[test]
    fn test_stochrsi_basic() {
        let close = [
            81.59, 81.06, 82.87, 83.00, 83.61, 83.15, 82.84, 83.99, 84.55, 84.36, 85.53, 86.54,
            86.89, 87.77, 87.29, 88.15, 89.44, 89.18, 88.67, 87.96, 89.43, 90.17, 90.78, 91.04,
            90.58, 91.32, 90.13, 89.14, 89.44, 88.43,
        ];
        let options = vec![14.0];

        let input_refs: [&[f64]; 1] = [&close];
        let options_array: [f64; 1] = [options[0]];

        let result = stochrsi_impl::indicator(&input_refs, &options_array, None);
        assert!(result.is_ok());
        let (outputs, _state) = result.unwrap();
        assert!(!outputs[0].is_empty());
    }

    #[test]
    fn test_stochrsi_state_continuation() {
        let close = [
            81.59, 81.06, 82.87, 83.00, 83.61, 83.15, 82.84, 83.99, 84.55, 84.36, 85.53, 86.54,
            86.89, 87.77, 87.29, 88.15, 89.44, 89.18, 88.67, 87.96, 89.43, 90.17, 90.78, 91.04,
            90.58, 91.32, 90.13, 89.14, 89.44, 88.43,
        ];
        let options = vec![14.0];

        // Test state continuation
        let split_point = 25;
        let input_refs1: [&[f64]; 1] = [&close[..split_point]];
        let input_refs2: [&[f64]; 1] = [&close[split_point..]];
        let options_array: [f64; 1] = [options[0]];

        let (_outputs1, mut state) =
            stochrsi_impl::indicator(&input_refs1, &options_array, None).unwrap();
        let outputs2 = state.batch_indicator(&input_refs2, None).unwrap();

        assert!(!outputs2[0].is_empty());
    }
}
