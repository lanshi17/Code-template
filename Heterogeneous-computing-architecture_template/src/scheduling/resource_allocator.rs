// Resource allocator implementation

pub struct ResourceAllocator {
    strategy: AllocationStrategy,
}

pub enum AllocationStrategy {
    FirstFit,
    BestFit,
    WorstFit,
}

impl ResourceAllocator {
    pub fn new(strategy: AllocationStrategy) -> Self {
        Self { strategy }
    }

    pub fn allocate_resources(&self, requirements: &ResourceRequirements, devices: &[&dyn HardwareDevice]) -> Option<&dyn HardwareDevice> {
        match self.strategy {
            AllocationStrategy::FirstFit => {
                // Allocate to the first device that meets requirements
                devices.iter().find(|device| {
                    device.available_memory() >= requirements.memory
                }).copied()
            },
            AllocationStrategy::BestFit => {
                // Allocate to the device that best fits the requirements
                let mut best_fit = None;
                let mut best_fit_size = u64::MAX;
                
                for device in devices {
                    let available_memory = device.available_memory();
                    if available_memory >= requirements.memory && available_memory < best_fit_size {
                        best_fit = Some(*device);
                        best_fit_size = available_memory;
                    }
                }
                
                best_fit
            },
            AllocationStrategy::WorstFit => {
                // Allocate to the device with the most available memory
                let mut worst_fit = None;
                let mut worst_fit_size = 0;
                
                for device in devices {
                    let available_memory = device.available_memory();
                    if available_memory >= requirements.memory && available_memory > worst_fit_size {
                        worst_fit = Some(*device);
                        worst_fit_size = available_memory;
                    }
                }
                
                worst_fit
            }
        }
    }
}