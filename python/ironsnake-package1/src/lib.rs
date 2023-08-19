extern crate ironsnake as ironsnake_rs; // External python crate in ../../crates/python

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn five() -> PyResult<i32> {
    Ok(ironsnake_rs::five())
}

#[pymodule]
fn ironsnake_package1_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(five, m)?)?;
    Ok(())
}
