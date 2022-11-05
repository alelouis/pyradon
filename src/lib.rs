mod array;
mod radon;

use pyo3::prelude::*;

/// radon transform function
#[pyfunction]
fn transform(
    image: Vec<Vec<f32>>,
    n_rows: usize,
    n_cols: usize,
    n_rays: usize,
    n_slopes: usize,
) -> PyResult<Vec<Vec<f32>>> {
    let radon_transform = radon::radon(image, n_rows, n_cols, n_rays, n_slopes);
    Ok(radon_transform)
}

/// pyradon module
#[pymodule]
fn pyradon(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(transform, m)?)?;
    Ok(())
}
