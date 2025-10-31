// Computation module

pub mod computation_engine;
pub mod task_executor;

// Re-export key types
pub use computation_engine::ComputationEngine;
pub use task_executor::TaskExecutor;

pub struct TaskResult {
    pub task_id: String,
    pub data: Vec<u8>,
    pub execution_time: u64,
}