use cpython::{PyResult, Python, py_module_initializer, py_fn};

py_module_initializer!(template, |py, m| {
    m.add(py, "__doc__", "docs goes here!")?;
    m.add(py, "test", py_fn!(py, test_func()))?;
    Ok(())
});

fn test_func(_py: Python) -> PyResult<&'static str> {
    Ok("Hello World")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        assert_eq!(test_func(py), PyResult::Ok("Hello World"));
    }
}

