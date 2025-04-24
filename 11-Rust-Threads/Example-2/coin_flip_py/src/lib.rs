use std::time::Instant;

use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};

use ::coin_flip::*;
use ::coin_flip::flip_task::FlipTask;

#[pyfunction]
fn do_flips(num_threads: usize, num_flips: u64) {
    if num_threads == 1 {
        let start = Instant::now();

        let result = FlipTask::simulate_flips(num_flips);
        println!("{}", result);

        let duration = start.elapsed();
        println!("{}", "-".repeat(72));
        println!("Sequential Time: {duration:?}");
    } else {
        let start = Instant::now();

        run_parallel(num_threads, num_flips);

        let duration = start.elapsed();
        println!("Parallel Time: {duration:?}");
    }
}

#[pymodule]
fn coin_flip_py(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(do_flips))?;
    Ok(())
}
