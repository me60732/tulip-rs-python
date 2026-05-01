//! Template file for creating new indicator bindings with auto-registration
//!
//! This file demonstrates the recommended pattern for creating PyO3 bindings
//! for tulip_rs indicators using the auto-registration macros.
//!
//! To create a new indicator binding:
//! 1. Copy this template
//! 2. Replace "template" with your indicator name throughout
//! 3. Update the imports to use your specific tulip_rs indicator
//! 4. Adjust the input/output handling for your indicator's specific requirements
//! 5. Add your register function call to lib.rs

use crate::utils::info_to_hashmap;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tulip_rs::indicator_types::TIndicatorState;
// Replace with your actual indicator import:
// use tulip_rs::indicators::your_indicator as rust_indicator;

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct TemplateState {
    // Replace with your actual indicator state:
    // inner: rust_indicator::IndicatorState,
}

#[pymethods]
impl TemplateState {
    #[pyo3(signature = (inputs, optional_outputs=None))]
    fn batch_indicator(
        &mut self,
        inputs: Vec<PyReadonlyArray1<f64>>,
        optional_outputs: Option<Vec<bool>>,
    ) -> PyResult<Vec<Vec<f64>>> {
        // Replace INPUTS_WIDTH with your indicator's constant:
        // if inputs.len() != rust_indicator::INPUTS_WIDTH {
        //     return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
        //         "Expected {} inputs, got {}",
        //         rust_indicator::INPUTS_WIDTH,
        //         inputs.len()
        //     )));
        // }

        // Adjust input array construction based on your indicator's needs:
        // For single input (like SMA):
        // let input_arrays: [&[f64]; 1] = [inputs[0].as_slice()?];

        // For multiple inputs (like AD with 4 inputs):
        // let input_arrays: [&[f64]; rust_indicator::INPUTS_WIDTH] = [
        //     inputs[0].as_slice()?,
        //     inputs[1].as_slice()?,
        //     inputs[2].as_slice()?,
        //     inputs[3].as_slice()?,
        // ];

        // Replace with your actual indicator call:
        // match self.inner.batch_indicator(&input_arrays, optional_outputs.as_deref()) {
        //     Ok(result) => Ok(result),
        //     Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
        //         "Indicator calculation failed: {:?}",
        //         e
        //     ))),
        // }

        // Placeholder return for template:
        Ok(vec![])
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
    inputs: Vec<PyReadonlyArray1<f64>>,
    options: Vec<f64>,
    optional_outputs: Option<Vec<bool>>,
) -> PyResult<(Vec<Vec<f64>>, TemplateState)> {
    // Replace with your indicator's validation:
    // if inputs.len() != rust_indicator::INPUTS_WIDTH {
    //     return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
    //         "Expected {} inputs, got {}",
    //         rust_indicator::INPUTS_WIDTH,
    //         inputs.len()
    //     )));
    // }

    // if options.len() != rust_indicator::OPTIONS_WIDTH {
    //     return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
    //         "Expected {} options, got {}",
    //         rust_indicator::OPTIONS_WIDTH,
    //         options.len()
    //     )));
    // }

    // Adjust input/options array construction:
    // let input_arrays: [&[f64]; rust_indicator::INPUTS_WIDTH] = [
    //     inputs[0].as_slice()?,
    //     // ... add more inputs as needed
    // ];

    // let options_array: [f64; rust_indicator::OPTIONS_WIDTH] =
    //     options.try_into().map_err(|_| {
    //         PyErr::new::<pyo3::exceptions::PyValueError, _>("Options conversion failed")
    //     })?;

    // Replace with your actual indicator call:
    // match rust_indicator::indicator(&input_arrays, &options_array, optional_outputs.as_deref()) {
    //     Ok((result, state)) => Ok((result, TemplateState { inner: state })),
    //     Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
    //         "Indicator calculation failed: {:?}",
    //         e
    //     ))),
    // }

    // Placeholder return for template:
    Ok((vec![], TemplateState {}))
}

#[pyfunction]
pub fn info() -> PyResult<HashMap<String, String>> {
    // Replace with your actual info call:
    // let info = rust_indicator::info();
    // Ok(info_to_hashmap(info))

    // Placeholder for template:
    let mut info = HashMap::new();
    info.insert("name".to_string(), "Template Indicator".to_string());
    Ok(info)
}

#[pyfunction]
pub fn min_data(options: Vec<f64>) -> PyResult<usize> {
    // Replace with your actual min_data call:
    // Ok(rust_indicator::min_data(&options))

    // Placeholder for template:
    Ok(1)
}

#[pyfunction]
pub fn min_data_accuracy(options: Vec<f64>, decimals: usize) -> PyResult<usize> {
    // Replace with your actual min_data_accuracy call:
    // Ok(rust_indicator::min_data_accuracy(&options, decimals))

    // Placeholder for template:
    Ok(1)
}

#[pyfunction]
pub fn output_length(data_len: usize, options: Vec<f64>) -> PyResult<usize> {
    // Replace with your actual output_length call:
    // Ok(rust_indicator::output_length(data_len, &options))

    // Placeholder for template:
    Ok(data_len)
}

// OPTION 1: Manual registration (current working approach)
// Use this approach for now until macros are fully implemented
pub fn register_template_module(parent_module: &pyo3::Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    let submodule = PyModule::new(parent_module.py(), "template")?;

    submodule.add_function(pyo3::wrap_pyfunction!(indicator, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(info, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(min_data_accuracy, &submodule)?)?;
    submodule.add_function(pyo3::wrap_pyfunction!(output_length, &submodule)?)?;
    submodule.add_class::<TemplateState>()?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}

// OPTION 2: Auto-registration using declarative macros (future approach)
// When the macros are properly imported and working, you can use:
//
// register_standard_indicator! {
//     name: template,
//     state: TemplateState
// }
//
// Or for indicators with additional functions:
// register_standard_indicator! {
//     name: template,
//     state: TemplateState,
//     additional_functions: [simd_by_assets, special_function]
// }
//
// Or for full control:
// register_indicator_module! {
//     name: template,
//     functions: [indicator, info, min_data, min_data_accuracy, output_length],
//     classes: [TemplateState]
// }

/*
HOW TO USE THIS TEMPLATE:

1. Copy this file to src/indicators/your_indicator.rs
2. Replace all instances of "template" with your indicator name
3. Replace all instances of "Template" with your indicator name (capitalized)
4. Update the imports to reference your actual tulip_rs indicator
5. Update the input/output validation and handling
6. Add your module to src/indicators/mod.rs:
   - Add `pub mod your_indicator;` at the top
   - Add a pub use statement if needed for re-exports
7. Add registration call to src/lib.rs in the tulip_rs function:
   - Add `indicators::your_indicator::register_your_indicator_module(&indicators_module)?;`
8. Test with `maturin develop --release`

EXAMPLE PATTERNS:

Single input indicator (like SMA):
```rust
let input_arrays: [&[f64]; 1] = [inputs[0].as_slice()?];
```

Multiple inputs (like AD with high, low, close, volume):
```rust
let input_arrays: [&[f64]; 4] = [
    inputs[0].as_slice()?,
    inputs[1].as_slice()?,
    inputs[2].as_slice()?,
    inputs[3].as_slice()?,
];
```

No options (like AD):
```rust
if options.len() != 0 {
    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
        format!("Expected 0 options, got {}", options.len())
    ));
}
let options_array: [f64; 0] = [];
```

Single option (like SMA period):
```rust
if options.len() != 1 {
    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
        format!("Expected 1 option, got {}", options.len())
    ));
}
let options_array: [f64; 1] = [options[0]];
```
*/
