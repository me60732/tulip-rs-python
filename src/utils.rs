use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tulip_rs::types::Info;

/// Converts a TulipRS [`Info`] struct into a Python dict.
///
/// The returned dict has the following shape:
/// ```python
/// {
///     "name": str,
///     "full_name": str,
///     "indicator_type": str,          # e.g. "Trend", "Momentum", …
///     "inputs":          list[str],
///     "options":         list[str],
///     "outputs":         list[str],
///     "optional_outputs": list[str],
///     "display_groups": [             # one entry per rendering group
///         {
///             "id":           str,    # stable machine-readable key
///             "label":        str,    # human-readable pane title
///             "display_type": str,    # "Overlay" | "Indicator" | "Volume"
///             "outputs":      list[str],
///         },
///         …
///     ],
/// }
/// ```
pub fn info_to_pydict<'py>(py: Python<'py>, info: Info) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);

    dict.set_item("name", info.name)?;
    dict.set_item("full_name", info.full_name)?;
    dict.set_item("indicator_type", format!("{:?}", info.indicator_type))?;

    // Scalar slices → Python lists
    dict.set_item("inputs", info.inputs.to_vec())?;
    dict.set_item("options", info.options.to_vec())?;
    dict.set_item("outputs", info.outputs.to_vec())?;
    dict.set_item("optional_outputs", info.optional_outputs.to_vec())?;

    // Build display_groups as a list of dicts
    let groups_list = PyList::empty(py);
    for group in info.display_groups {
        let g = PyDict::new(py);
        g.set_item("id", group.id)?;
        g.set_item("label", group.label)?;
        g.set_item("display_type", format!("{:?}", group.display_type))?;
        g.set_item("outputs", group.outputs.to_vec())?;
        groups_list.append(g)?;
    }
    dict.set_item("display_groups", groups_list)?;

    Ok(dict)
}
