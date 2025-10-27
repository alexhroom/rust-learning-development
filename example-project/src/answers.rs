use faer_ext::IntoFaer;
use pyo3::prelude::*;
use numpy::PyReadonlyArray2;

/// Formats the sum of a list of numbers as string.
#[pyfunction]
fn list_sum_string(nums: Vec<f64>) -> String {
    nums.into_iter().sum::<f64>().to_string()
}

/// Formats the sum of a 2D array of numbers as string.
#[pyfunction]
fn array_sum_string(nums: PyReadonlyArray2<f64>) -> String {
    let array = nums.into_faer();
    array.sum().to_string()
}

/// A Python module implemented in Rust.
#[pymodule]
fn example_project(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_sum_string, m)?)?;
    m.add_function(wrap_pyfunction!(array_sum_string, m)?)?;
    Ok(())
}
