//! Task scheduling implementation

use crate::hardware::abstract::{HardwareManager, TaskRequirements, HardwareType};
use crate::scheduling::resource_allocator::ResourceAllocator;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::debug;

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Critical = 100,
    High = 75,
    Normal = 50,
    Low = 25,
    Background = 10,
}

/// Task structure
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub requirements: TaskRequirements,
    pub priority: TaskPriority,
    pub submission_time: u64,
    pub status: TaskStatus,
}

/// Task status enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskStatus {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl Task {
    /// Create a new task
    pub fn new(
        id: String,
        name: String,
        requirements: TaskRequirements,
        priority: TaskPriority,
    ) -> Self {
        Self {
            id,
            name,
            requirements,
            priority,
            submission_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            status: TaskStatus::Pending,
        }
    }
}

/// Task scheduler implementation
pub struct TaskScheduler {
    task_queue: Arc<std::sync::Mutex<VecDeque<Task>>>,
    resource_allocator: Arc<ResourceAllocator>,
    next_task_id: AtomicU64,
}

impl TaskScheduler {
    /// Create a new task scheduler
    pub fn new(resource_allocator: Arc<ResourceAllocator>) -> Self {
        Self {
            task_queue: Arc::new(std::sync::Mutex::new(VecDeque::new())),
            resource_allocator,
            next_task_id: AtomicU64::new(1),
        }
    }
    
    /// Submit a new task to the scheduler
    pub fn submit_task(&self, task: Task) -> Result<String, Box<dyn std::error::Error>> {
        debug!("Submitting task: {}", task.name);
        
        // Add task to queue
        self.task_queue.lock().unwrap().push_back(task);
        
        // Return the assigned task ID
        let task_id = self.next_task_id.fetch_add(1, Ordering::Relaxed).to_string();
        Ok(task_id)
    }
    
    /// Schedule the next available task
    pub fn schedule_next_task(&self) -> Option<Task> {
        debug!("Attempting to schedule next task");
        
        // Lock the task queue
        let mut queue = self.task_queue.lock().unwrap();
        
        // Find the highest priority task that can be scheduled
        let mut highest_priority_index = None;
        let mut highest_priority = TaskPriority::Background;
        
        for (index, task) in queue.iter().enumerate() {
            // Check if the task can be scheduled with current resources
            if task.priority >= highest_priority {
                highest_priority = task.priority;
                highest_priority_index = Some(index);
            }
        }
        
        // If a task was found, remove and return it
        if let Some(index) = highest_priority_index {
            Some(queue.remove(index).unwrap())
        } else {
            None
        }
    }
    
    /// Get pending tasks count
    pub fn get_pending_tasks_count(&self) -> usize {
        self.task_queue.lock().unwrap().len()
    }
    
    /// Cancel a task by ID
    pub fn cancel_task(&self, task_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Cancelling task: {}", task_id);
        
        // Remove the task from the queue
        let mut queue = self.task_queue.lock().unwrap();
        queue.retain(|task| task.id != task_id);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_creation() {
        let requirements = TaskRequirements {
            hardware_type: HardwareType::Cpu,
            memory_required: 1024,
            compute_units: 1,
            priority: 50,
            timeout: 300000,
        };
        
        let task = Task::new(
            "task-1".to_string(),
            "Test task".to_string(),
            requirements,
            TaskPriority::Normal,
        );
        
        assert_eq!(task.id, "task-1");
        assert_eq!(task.name, "Test task");
        assert_eq!(task.requirements.hardware_type, HardwareType::Cpu);
        assert_eq!(task.priority, TaskPriority::Normal);
    }
    
    #[test]
    fn test_task_prioritization() {
        let requirements1 = TaskRequirements {
            hardware_type: HardwareType::Cpu,
            memory_required: 1024,
            compute_units: 1,
            priority: 50,
            timeout: 300000,
        };
        
        let requirements2 = TaskRequirements {
            hardware_type: HardwareType::Gpu,
            memory_required: 2048,
            compute_units: 2,
            priority: 75,
            timeout: 300000,
        };
        
        let task1 = Task::new(
            "task-1".to_string(),
            "Low priority task".to_string(),
            requirements1,
            TaskPriority::Low,
        );
        
        let task2 = Task::new(
            "task-2".to_string(),
            "High priority task".to_string(),
            requirements2,
            TaskPriority::High,
        );
        
        assert!(task2.priority > task1.priority);
    }
}