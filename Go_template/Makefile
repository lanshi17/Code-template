# Project-specific variables
PROJECT_NAME := Go_template
BINARY_NAME := $(PROJECT_NAME)
MAIN_FILE := ./cmd/$(PROJECT_NAME)/main.go

# Go parameters
GOCMD := go
GOBUILD := $(GOCMD) build
GOCLEAN := $(GOCMD) clean
GOTEST := $(GOCMD) test
GOGET := $(GOCMD) get
GOMOD := $(GOCMD) mod
GOFMT := $(GOCMD) fmt
GOVET := $(GOCMD) vet
GOLINT := golangci-lint

# Build directory
BUILD_DIR := bin

# Default target
all: build

# Build the binary
build:
	$(GOBUILD) -o $(BUILD_DIR)/$(BINARY_NAME) -v $(MAIN_FILE)

# Install dependencies
deps:
	$(GOMOD) tidy
	$(GOMOD) download

# Clean build artifacts
clean:
	$(GOCLEAN)
	rm -rf $(BUILD_DIR)

# Run tests
test:
	$(GOTEST) -v ./...

# Run tests with coverage
test-cover:
	$(GOTEST) -coverprofile=coverage.out ./...
	$(GOCMD) tool cover -html=coverage.out

# Format source code
fmt:
	$(GOFMT) ./...

# Vet checks
vet:
	$(GOVET) ./...

# Lint checks
lint:
	$(GOLINT) run

# Run the application
run: build
	./$(BUILD_DIR)/$(BINARY_NAME)

# Run the application with race detector
run-race: build
	$(GOCMD) run -race $(MAIN_FILE)

# Install development tools
install-tools:
	$(GOGET) github.com/golangci/golangci-lint/cmd/golangci-lint@latest

# Docker build
docker-build:
	docker build -t $(PROJECT_NAME) .

# Docker run
docker-run:
	docker run -p 8080:8080 $(PROJECT_NAME)

# Help target
help:
	@echo "Usage: make [target]"
	@echo
	@echo "Targets:"
	@echo "  all             Build the binary (default)"
	@echo "  build           Build the binary"
	@echo "  deps            Install dependencies"
	@echo "  clean           Clean build artifacts"
	@echo "  test            Run tests"
	@echo "  test-cover      Run tests with coverage"
	@echo "  fmt             Format source code"
	@echo "  vet             Vet checks"
	@echo "  lint            Lint checks"
	@echo "  run             Run the application"
	@echo "  run-race        Run the application with race detector"
	@echo "  install-tools   Install development tools"
	@echo "  docker-build    Build Docker image"
	@echo "  docker-run      Run Docker container"
	@echo "  help            Show this help"

.PHONY: all build deps clean test test-cover fmt vet lint run run-race install-tools docker-build docker-run help