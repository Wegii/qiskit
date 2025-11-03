pub(crate) mod route;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// pub(crate) use route::basic_routing;

pub fn basic_swap(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(route::basic_routing))?;
    Ok(())
}
