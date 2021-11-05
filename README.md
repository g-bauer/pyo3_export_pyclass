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
  = note: Undefined symbols for architecture x86_64:
    "_PyType_GetFlags", referenced from:
        pyo3::err::err_state::PyErrState::into_ffi_tuple::h0262743d0d58ede8 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.14.rcgu.o)
    "_PyExc_TypeError", referenced from:
        pyo3::err::err_state::PyErrState::into_ffi_tuple::h0262743d0d58ede8 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.14.rcgu.o)
    "__Py_Dealloc", referenced from:
        pyo3::gil::ReferencePool::update_counts::h5e8394223f659efd in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.8.rcgu.o)
        _$LT$pyo3..gil..GILPool$u20$as$u20$core..ops..drop..Drop$GT$::drop::h348136bec82eceda in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.8.rcgu.o)
        pyo3::gil::register_decref::h2deffc4062f7a252 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.8.rcgu.o)
    "_PyTuple_New", referenced from:
        core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hbc6b9f581ebb9e3f (.llvm.13551205879325873265) in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.14.rcgu.o)
        _$LT$T$u20$as$u20$pyo3..err..err_state..PyErrArguments$GT$::arguments::h66f3a1b343fc3be1 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.1.rcgu.o)
    "_PyUnicode_FromStringAndSize", referenced from:
        core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h33c3245faa74663d (.llvm.13551205879325873265) in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.14.rcgu.o)
        core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hbc6b9f581ebb9e3f (.llvm.13551205879325873265) in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.14.rcgu.o)
        core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::he035dbb0ae7839a8 (.llvm.13551205879325873265) in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.14.rcgu.o)
        _$LT$T$u20$as$u20$pyo3..err..err_state..PyErrArguments$GT$::arguments::h66f3a1b343fc3be1 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.1.rcgu.o)
    "_PyExc_BaseException", referenced from:
        _$LT$T$u20$as$u20$pyo3..type_object..PyTypeObject$GT$::type_object::h2fa27bc99ffb5f18 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.2.rcgu.o)
    "_PyTuple_SetItem", referenced from:
        core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hbc6b9f581ebb9e3f (.llvm.13551205879325873265) in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.14.rcgu.o)
        _$LT$T$u20$as$u20$pyo3..err..err_state..PyErrArguments$GT$::arguments::h66f3a1b343fc3be1 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.1.rcgu.o)
    "_PyErr_NewException", referenced from:
        pyo3::err::PyErr::new_type::h2ee8b74f2cbed4e8 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.3.rcgu.o)
    "__Py_NoneStruct", referenced from:
        _$LT$$LP$$RP$$u20$as$u20$pyo3..conversion..IntoPy$LT$pyo3..instance..Py$LT$pyo3..types..any..PyAny$GT$$GT$$GT$::into_py::h1d254e707a36f47c in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.2.rcgu.o)
    "_PyErr_Print", referenced from:
        pyo3::err::panic_after_error::h1e687e2565badad9 in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.3.rcgu.o)
    "_PyExc_RuntimeError", referenced from:
        _$LT$T$u20$as$u20$pyo3..type_object..PyTypeObject$GT$::type_object::h1a4afc76e28f7baa in libpyo3-edb541318369fe53.rlib(pyo3-edb541318369fe53.pyo3.dnu2vjgj-cgu.0.rcgu.o)
    "_PyErr_Restore", referenced from:
        pyo3_export_pyclass::__init11128182787261008535::__wrap::h1224a1983a2eeb71 in pyo3_export_pyclass.pyo3_export_pyclass.c9cqmf0q-cgu.3.rcgu.o
  ld: symbol(s) not found for architecture x86_64
  clang: error: linker command failed with exit code 1 (use -v to see invocation)
```