# Heterogeneous Computing Architecture

This project implements a flexible architecture for heterogeneous computing systems that can utilize CPU, GPU, and FPGA resources efficiently.

## Overview

The heterogeneous computing architecture provides a unified framework for managing and scheduling computations across different types of hardware accelerators. It enables efficient utilization of diverse compute resources for machine learning, scientific computing, and data processing tasks.

## Features

- Hardware abstraction layer supporting CPU, GPU, and FPGA
- Dynamic resource allocation and task scheduling
- Model management and optimization
- Distributed computing capabilities
- Monitoring and metrics collection
- Containerized deployment with Docker

## Architecture

The system consists of several key modules:

1. **Hardware Abstraction Layer**: Provides unified interfaces to different hardware types
2. **Task Scheduler**: Distributes computational tasks across available resources
3. **Resource Allocator**: Manages allocation and deallocation of computing resources
4. **Computation Engine**: Executes tasks on appropriate hardware
5. **Model Manager**: Handles model loading, optimization, and deployment
6. **Data Management**: Processes data transformations and transfers
7. **Utilities**: Logging, metrics, and error handling

## Directory Structure

```
.
├── Cargo.toml                 # Rust package manifest
├── Dockerfile                 # Docker build configuration
├── docker-compose.yml         # Docker compose configuration
├── Makefile                   # Build automation
├── README.md                  # This file
├── config/                    # Configuration files
│   ├── application.yaml       # Main application config
│   ├── hardware/              # Hardware-specific configs
│   │   ├── cpu.yaml
│   │   ├── gpu.yaml
│   │   └── fpga.yaml
│   └── scheduling/            # Scheduling configurations
│       ├── task_scheduling.yaml
│       └── resource_allocation.yaml
├── src/                       # Source code
│   ├── main.rs                # Entry point
│   ├── hardware/              # Hardware abstraction implementations
│   │   ├── abstract/          # Abstract base implementations
│   │   ├── cpu/               # CPU-specific implementations
│   │   ├── gpu/               # GPU-specific implementations
│   │   └── fpga/              # FPGA-specific implementations
│   ├── scheduling/            # Scheduling logic
│   ├── computation/           # Computation engine
│   ├── data/                  # Data management
│   ├── models/                # Model handling
│   ├── utils/                 # Utilities
│   └── errors/                # Custom error types
├── models/                    # Pre-trained models
│   └── ai/                    # Machine learning models
│       ├── llm/               # Large language models
│       │   ├── llama3/        # LLaMA3 models
│       │   └── qwen/          # Qwen models
│       └── embeddings/        # Embedding models
│           ├── text_embedding/ # Text embedding models
│           └── image_embedding/ # Image embedding models
├── scripts/                   # Deployment and utility scripts
├── docs/                      # Documentation
└── tests/                     # Unit and integration tests
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Docker (optional, for containerized deployment)

### Building

```bash
make build
```

### Running

```bash
make run
```

### Testing

```bash
make test
```

### Linting

```bash
make lint
```

### Formatting

```bash
make format
```

## Configuration

### Application Configuration

Main application settings are defined in `config/application.yaml`.

### Hardware Configurations

Each hardware type has its own configuration file:
- `config/hardware/cpu.yaml` - CPU settings
- `config/hardware/gpu.yaml` - GPU settings
- `config/hardware/fpga.yaml` - FPGA settings

### Scheduling Configuration

Scheduling behavior is controlled by:
- `config/scheduling/task_scheduling.yaml` - Task scheduling policies
- `config/scheduling/resource_allocation.yaml` - Resource allocation strategies

## Deployment

### Local Development

```bash
docker-compose up
```

### Production Deployment

```bash
make deploy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Rust community for the excellent ecosystem
- NVIDIA for CUDA technology
- Xilinx for FPGA solutions