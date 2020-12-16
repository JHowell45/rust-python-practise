use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// the docs is in a comment above the pyfunction!
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
	Ok((a + b).to_string())
}

/// the docs is in a comment above the pymodule!
#[pymodule]
fn template(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

	Ok(())
}
