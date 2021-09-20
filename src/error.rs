use pyo3::exceptions::PyException;

pyo3::import_exception!(builtins, ValueError);
pyo3::import_exception!(builtins, ConnectionError);
pyo3::import_exception!(builtins, TimeoutError);
pyo3::import_exception!(builtins, TypeError);
pyo3::import_exception!(builtins, Exception);
pyo3::import_exception!(builtins, NameError);
pyo3::import_exception!(ipaddress, AddressValueError);
pyo3::create_exception!(lavasnek_rs, NoSessionPresent, PyException);
pyo3::create_exception!(lavasnek_rs, NetworkError, PyException);
