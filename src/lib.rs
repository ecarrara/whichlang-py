use pyo3::prelude::*;

#[pyfunction]
fn detect_language(text: String) -> &'static str {
    let lang = whichlang::detect_language(&text);
    lang.three_letter_code()
}

#[pymodule]
fn whichlang_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(detect_language, m)?)?;
    Ok(())
}
