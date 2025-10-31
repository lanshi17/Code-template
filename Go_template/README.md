# Go Template Project

A template project for Go applications with a clean architecture.

## Features

- Clean Architecture
- RESTful API with Gin
- Configuration management with Viper
- Docker support
- Docker Compose for development
- Makefile for common tasks
- Go modules
- Testing setup
- Code formatting and linting
- CI/CD ready

## Getting Started

### Prerequisites

- Go 1.22 or higher
- Docker (optional, for containerization)
- Docker Compose (optional, for development)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd Go_template
   ```

2. Install dependencies:
   ```bash
   make deps
   ```

### Running the Application

#### Local Development

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Run the application:
   ```bash
   make run
   ```

#### Using Docker

1. Build the Docker image:
   ```bash
   make docker-build
   ```

2. Run the Docker container:
   ```bash
   make docker-run
   ```

#### Using Docker Compose

1. Start the services:
   ```bash
   docker-compose up
   ```

### Testing

Run tests:
```bash
make test
```

Run tests with coverage:
```bash
make test-cover
```

### Code Quality

Format the code:
```bash
make fmt
```

Run vet checks:
```bash
make vet
```

Run lint checks:
```bash
make lint
```

### Project Structure

```
Go_template/
├── .env.example                # Environment variables example
├── .gitignore
├── .golangci.yml               # Go code static analysis configuration
├── Dockerfile
├── docker-compose.yml
├── Makefile
├── go.mod                      # Go module file
├── go.sum                      # Go module dependencies
├── LICENSE                     # Project license
├── README.md                   # Project documentation
├── cmd/                        # Project main entry points
│   └── Go_template/            # Main executable
│       └── main.go
├── config/                     # Configuration management
│   ├── config.go               # Configuration loading
│   ├── config.yaml             # Configuration example
│   └── config_test.go          # Configuration tests
├── internal/                   # Private application code
│   ├── app/                    # Application initialization
│   │   ├── app.go
│   │   └── app_test.go
│   ├── handler/                # HTTP handlers
│   │   ├── handler.go
│   │   └── handler_test.go
│   ├── service/                # Business services
│   │   ├── service.go
│   │   └── service_test.go
│   └── repository/             # Data access
│       ├── repository.go
│       └── repository_test.go
├── pkg/                        # Public libraries
│   ├── util/                   # Utility functions
│   │   ├── util.go
│   │   └── util_test.go
│   └── errors/                 # Custom errors
│       ├── errors.go
│       └── errors_test.go
├── api/                        # API definitions
│   └── openapi.yaml            # OpenAPI specification
├── docs/                       # Documentation
│   ├── api_reference/          # API reference
│   │   └── swagger-ui.html
│   └── design.md               # Architecture design document
├── scripts/                    # Scripts
│   ├── build.sh                # Build script
│   ├── run.sh                  # Run script
│   └── test.sh                 # Test script
├── tests/                      # Tests
│   ├── integration/            # Integration tests
│   │   └── integration_test.go
│   └── unit/                   # Unit tests
│       └── unit_test.go
└── vendor/                     # Dependencies (less used with Go Modules)
```

### API Documentation

API documentation is available in the `api/` directory in OpenAPI format.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.