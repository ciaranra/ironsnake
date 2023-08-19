use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from python.ironsnake-package2_pyo3!".to_string())
}

#[pymodule]
fn ironsnake_package2_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;

    Ok(())
}
