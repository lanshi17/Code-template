// Task scheduler implementation

use std::collections::VecDeque;
use crate::hardware::abstract_device::HardwareDevice;

pub struct TaskScheduler {
    tasks: VecDeque<Task>,
    strategy: SchedulingStrategy,
}

#[derive(Clone)]
pub enum SchedulingStrategy {
    RoundRobin,
    Priority,
    ShortestJobFirst,
}

pub struct Task {
    pub id: String,
    pub priority: u8,
    pub estimated_duration: u64,
    pub required_resources: ResourceRequirements,
}

pub struct ResourceRequirements {
    pub cpu_cores: usize,
    pub memory: u64,
    pub device_type: String,
}

impl TaskScheduler {
    pub fn new(strategy: SchedulingStrategy) -> Self {
        Self {
            tasks: VecDeque::new(),
            strategy,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    pub fn get_next_task(&mut self) -> Option<Task> {
        match self.strategy {
            SchedulingStrategy::RoundRobin => self.tasks.pop_front(),
            SchedulingStrategy::Priority => {
                // Find the task with the highest priority
                let mut max_priority = 0;
                let mut max_index = None;
                
                for (i, task) in self.tasks.iter().enumerate() {
                    if task.priority > max_priority {
                        max_priority = task.priority;
                        max_index = Some(i);
                    }
                }
                
                if let Some(index) = max_index {
                    self.tasks.remove(index)
                } else {
                    self.tasks.pop_front()
                }
            },
            SchedulingStrategy::ShortestJobFirst => {
                // Find the task with the shortest estimated duration
                let mut min_duration = u64::MAX;
                let mut min_index = None;
                
                for (i, task) in self.tasks.iter().enumerate() {
                    if task.estimated_duration < min_duration {
                        min_duration = task.estimated_duration;
                        min_index = Some(i);
                    }
                }
                
                if let Some(index) = min_index {
                    self.tasks.remove(index)
                } else {
                    self.tasks.pop_front()
                }
            }
        }
    }
}