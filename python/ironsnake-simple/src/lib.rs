extern crate ironsnake_simple as ironsnake_rs; // External python crate in ../../crates/
use ironsnake_rs::Aggregate;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from ironsnake-simple!".to_string())
}

#[pyfunction]
fn five() -> PyResult<i32> {
    Ok(ironsnake_rs::five())
}

#[pyfunction]
pub fn get_tuple() -> PyResult<(String, i32, f64)> {
    Ok(ironsnake_rs::create_tuple())
}

#[pyclass]
pub struct PyAggregate {
    inner: Aggregate,
}

#[pymethods]
impl PyAggregate {
    #[new]
    pub fn new() -> Self {
        PyAggregate {
            inner: Aggregate::new(),
        }
    }

    #[getter]
    fn get_int(&self) -> PyResult<i32> {
        Ok(self.inner.int)
    }

    #[getter]
    fn get_float_number(&self) -> PyResult<f64> {
        Ok(self.inner.float_number)
    }

    #[getter]
    fn get_text(&self) -> PyResult<&str> {
        Ok(&self.inner.text)
    }

    #[getter]
    fn get_list(&self) -> PyResult<Vec<f64>> {
        Ok(self.inner.list.clone())
    }

    #[getter]
    fn get_tuple(&self) -> PyResult<(bool, i64)> {
        Ok(self.inner.tuple_data)
    }

    #[getter]
    fn get_map(&self) -> PyResult<HashMap<String, i32>> {
        Ok(self.inner.map.clone())
    }
}

// This way of bringing in an object keeps the connection between Rust and Python.
// Each time an attribute is access it is translated over to Python... you can also set values
// which translates Python data to the Rust object.
// There are different options... can use get and set methods... cache results... share memory...
// copying over information to the Python side for a one directional tranfer of info...
#[pyfunction]
pub fn aggregate_data() -> PyResult<PyAggregate> {
    Ok(PyAggregate::new())
}

// This example grabs information from the Rust side and does not use the get/set methods, which
// would allow Rust and Python to work together. This person example is about a single directional
// transfer of data.
#[pyfunction]
fn get_person() -> (String, u32) {
    let person = ironsnake_rs::generate_person();
    (person.name, person.age)
}

#[pymodule]
fn ironsnake_simple_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(five, m)?)?;
    m.add_function(wrap_pyfunction!(get_tuple, m)?)?;
    m.add_function(wrap_pyfunction!(aggregate_data, m)?)?;
    m.add_class::<PyAggregate>()?;
    m.add_function(wrap_pyfunction!(get_person, m)?)?;
    Ok(())
}
