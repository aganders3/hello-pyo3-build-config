
use pyo3::prelude::*;

/// Prints "Hello, World!"
#[pyfunction]
fn say_hello() {
    println!("Hello, World!");
}

/// A Python module implemented in Rust.
#[pymodule]
fn hello_pyo3_build_config(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}
