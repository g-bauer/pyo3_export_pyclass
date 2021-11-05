# pyO3 `PyClass` issues.

**Goal**: Have a crate (`py_export_pyclass`) that defines `PyClass` object(s) that can be used in a different crate (`python-package`) to generate a python wheel.

**Encountered problems**

- On Linux, everything works.
- On macOS, the following has to be true to work:
  - The crate that defines the `PyClass` must not have a `#[pymodule]` function. This can be controlled with a feature (`python_module`).
  - On macOS (not tested on Windows) if the `PyClass` in `src/lib.rs` must not define a method (`asd`).

The code in this repository will not work on macOS (not tested on windows).
Remove the method `asd` in `src/lib.rs` and it works.

## Build wheel

From the project root, type

```
maturin build --release -m python-package/Cargo.toml
```

(uses `-C link-arg=-undefined` and `-C link-arg=dynamic_lookup` in `cargo rustc`)

## Error Message (macOS)

```
error: linking with `cc` failed: exit status: 19: pyo3_export_pyclass
Undefinded symbols for architecture x86_64:

```