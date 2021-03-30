use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use indicatif::ProgressIterator;


/// the docs is in a comment above the pyfunction!
#[pyfunction]
fn estimate(number_of_points: i32) -> PyResult<f64> {
	let mut rng = thread_rng();
	let mut number_on_inside: i32 = 0;
	for _ in (1..=number_of_points).progress() {
		let first: f32 = rng.sample(Uniform::new(0.0, 1.0));
		let second: f32 = rng.sample(Uniform::new(0.0, 1.0));
		if first.powf(2.0) + second.powf(2.0) < 1.0 {
			number_on_inside += 1;
		}
	}
	let result: f64 = 4.0 * f64::from(number_on_inside) / f64::from(number_of_points);
	Ok(result)
}

/// the docs is in a comment above the pymodule!
#[pymodule]
fn estimate_pi(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(estimate, m)?)?;

	Ok(())
}
