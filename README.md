# rust-python-practise
Trying to get to grips with Python and Rust.

### How to build a template rust library:
The command for building a rust-python lib can be found here: https://gist.github.com/JHowell45/3b6b55aed622df81025cc8ae86083a69

In case it gets deleted or removed the current version looks like the following:
```buildoutcfg
function create-rust-python-lib() {
  TEMP=$(pwd)
  NAME=$1
  TITLE=${NAME/_/-}
  mkdir $NAME
  cd $NAME
  cargo new $NAME --lib

  # Create Cargo config:
  mkdir .cargo
  echo "[target.x86_64-apple-darwin]\nrustflags = [\n  \"-C\", \"link-arg=-undefined\",\n  \"-C\", \"link-arg=dynamic_lookup\",\n]" > .cargo/config

  # Create build script:
  echo "cd ${NAME}/\ncargo build --release\ncd ..\nmv ${NAME}/target/release/lib${NAME}.dylib ${NAME}.so" > build_mac.sh
  chmod +x build_mac.sh
  echo "cd ${NAME}/\ncargo build --release\ncd ..\nmv ${NAME}/target/release/lib${NAME}.so ${NAME}.so" > build_linux.sh
  chmod +x build_linux.sh
  echo "cd ${NAME}/\ncargo build --release\ncd ..\nmv ${NAME}/target/release/lib${NAME}.dll ${NAME}.pyd" > build_win.sh
  chmod +x build_win.sh

  # Update Cargo.toml file:
  cd $NAME
  echo "[package]\nname = \"${TITLE}\"\nversion = \"0.1.0\"\nedition = \"2018\"\n\n[lib]\nname = \"${NAME}\"\ncrate-type = [\"cdylib\"]\n\n[dependencies.pyo3]\nversion = \"0.12.4\"\nfeatures = [\"extension-module\"]" > Cargo.toml

  # Create template lib.rs file:
  cd src
  echo "use pyo3::prelude::*;\nuse pyo3::wrap_pyfunction;\n\n/// the docs is in a comment above the pyfunction!\n#[pyfunction]\nfn sum_as_string(a: usize, b: usize) -> PyResult<String> {\n\tOk((a + b).to_string())\n}\n\n/// the docs is in a comment above the pymodule!\n#[pymodule]\nfn ${NAME}(_py: Python, m: &PyModule) -> PyResult<()> {\n\tm.add_function(wrap_pyfunction!(sum_as_string, m)?)?;\n\n\tOk(())\n}" > lib.rs

  cd $TEMP
}
```
