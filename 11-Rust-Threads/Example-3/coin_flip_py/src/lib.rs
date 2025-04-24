use std::time::{Duration, Instant};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use ::coin_flip::flip_task::FlipTask;
use ::coin_flip::*;

#[pyclass]
pub struct FlipResult {
    task: FlipTask,
}

impl From<FlipTask> for FlipResult {
    fn from(src: FlipTask) -> Self {
        FlipResult { task: src }
    }
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq)]
pub enum SimulationMode {
    Sequential,
    Parallel,
}

#[pyclass]
pub struct FlipSummary {
    mode: SimulationMode,
    duration: Duration,
    results: Vec<FlipResult>,
    overall: FlipResult,
}

#[pymethods]
impl FlipSummary {
    fn __str__(&self) -> PyResult<String> {
        let summary = self
            .results
            .iter()
            .enumerate()
            .map(|(idx, result)| -> String { format!("Worker {idx:>2} -> {:}", result.task) })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(format!(
            "{}\n{}\n{}\n{}",
            summary,
            "-".repeat(72),
            format_args!("Overall   -> {}", self.overall.task),
            format_args!(
                "{}: {:?}",
                match self.mode {
                    SimulationMode::Sequential => "Sequential Time",
                    SimulationMode::Parallel => "Parallel Time",
                },
                self.duration
            )
        ))
    }
}

#[pyfunction]
fn do_flips(num_threads: usize, num_flips: u64) -> FlipSummary {
    let start = Instant::now();
    let (overall, results) = if num_threads == 1 {
        let result = FlipTask::simulate_flips(num_flips);

        (result, vec![result])
    } else {
        run_parallel(num_threads, num_flips)
    };
    let duration = start.elapsed();

    FlipSummary {
        mode: SimulationMode::Parallel,
        duration,
        results: results.into_iter().map(FlipResult::from).collect(),
        overall: overall.into(),
    }
}

#[pymodule(name = "coin_flip")]
fn coin_flip_py(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(do_flips))?;
    Ok(())
}
