use cpython::{PyResult, PyDict, PyErr, exc, Python, py_module_initializer, py_fn};


py_module_initializer!(tree, |py, m| {
    m.add(py, "__doc__", "docs goes here!")?;
    m.add(py, "test", py_fn!(py, test()))?;
    Ok(())
});

fn test() -> PyResult<String> {
    Ok("Hello World")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
