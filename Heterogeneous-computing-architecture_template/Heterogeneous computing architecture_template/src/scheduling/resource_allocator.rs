//! Resource allocation implementation

use crate::hardware::abstract::{HardwareManager, HardwareType, TaskRequirements};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::debug;

/// Resource allocation statistics
#[derive(Debug, Clone)]
pub struct AllocationStats {
    pub total_allocated: u64,
    pub total_available: u64,
    pub utilization_percentage: f64,
}

/// Resource allocator implementation
pub struct ResourceAllocator {
    hardware_manager: Arc<dyn HardwareManager>,
    allocations: Arc<std::sync::Mutex<HashMap<String, Vec<String>>>>,
    total_resources: AtomicU64,
    allocated_resources: AtomicU64,
}

impl ResourceAllocator {
    /// Create a new resource allocator
    pub fn new(hardware_manager: Arc<dyn HardwareManager>) -> Self {
        Self {
            hardware_manager,
            allocations: Arc::new(std::sync::Mutex::new(HashMap::new())),
            total_resources: AtomicU64::new(0),
            allocated_resources: AtomicU64::new(0),
        }
    }
    
    /// Allocate resources for a task
    pub fn allocate_resources(&self, task_requirements: &TaskRequirements) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        debug!(
            "Allocating resources for task requiring {:?} with {} MB memory",
            task_requirements.hardware_type,
            task_requirements.memory_required
        );
        
        // In a real implementation, this would:
        // 1. Check available resources
        // 2. Select appropriate devices
        // 3. Reserve resources
        // 4. Return resource IDs
        
        // For now, we'll simulate allocation
        let resource_ids = vec![
            format!("resource-{}-{}", task_requirements.hardware_type, self.allocated_resources.fetch_add(1, Ordering::Relaxed))
        ];
        
        // Record the allocation
        {
            let mut allocations = self.allocations.lock().unwrap();
            allocations.insert("task-1".to_string(), resource_ids.clone());
        }
        
        self.allocated_resources.fetch_add(1, Ordering::Relaxed);
        
        Ok(resource_ids)
    }
    
    /// Release allocated resources
    pub fn release_resources(&self, resource_ids: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Releasing resources: {:?}", resource_ids);
        
        // In a real implementation, this would:
        // 1. Free up reserved resources
        // 2. Update allocation records
        
        // For now, we'll just decrement the counter
        self.allocated_resources.fetch_sub(resource_ids.len() as u64, Ordering::Relaxed);
        
        // Remove allocation records
        {
            let mut allocations = self.allocations.lock().unwrap();
            allocations.retain(|_, v| !v.iter().any(|id| resource_ids.contains(id)));
        }
        
        Ok(())
    }
    
    /// Get allocation statistics
    pub fn get_stats(&self) -> AllocationStats {
        let total = self.total_resources.load(Ordering::Relaxed);
        let allocated = self.allocated_resources.load(Ordering::Relaxed);
        let utilization_percentage = if total > 0 {
            (allocated as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        
        AllocationStats {
            total_allocated: allocated,
            total_available: total,
            utilization_percentage,
        }
    }
    
    /// Get available resources of a specific type
    pub fn get_available_resources(&self, hardware_type: HardwareType) -> Vec<String> {
        // In a real implementation, this would query the hardware manager
        // for available devices of the specified type
        
        // For now, return a mock list
        vec![
            format!("available-resource-{}-0", hardware_type),
            format!("available-resource-{}-1", hardware_type),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_allocation_stats() {
        let stats = AllocationStats {
            total_allocated: 100,
            total_available: 1000,
            utilization_percentage: 10.0,
        };
        
        assert_eq!(stats.total_allocated, 100);
        assert_eq!(stats.total_available, 1000);
        assert_eq!(stats.utilization_percentage, 10.0);
    }
}