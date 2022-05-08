use pyo3::prelude::*;

/// This function takes a string input and delivers a tokenized
/// output to the user.
#[pyfunction]
fn tokenize(input: &str) -> PyResult<Vec<&str>> {
    Ok(cribrum::tokenize(input).into_iter().collect())
}

/// This module contains the wrappers of the Rust cribrum example.
#[pymodule]
fn cribrum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    Ok(())
}
