.PHONY: build run test lint format clean deploy

# Default target
all: build

# Build the application
build:
	cargo build --release

# Run the application
run:
	cargo run --release

# Run tests
test:
	cargo test

# Run linter
lint:
	cargo clippy

# Format code
format:
	cargo fmt

# Clean build artifacts
clean:
	cargo clean

# Deploy to production
deploy:
	docker build -t heterogeneous-architecture .
	docker push heterogeneous-architecture

# Run integration tests
integration-test:
	cargo test --features integration

# Run benchmarks
bench:
	cargo bench

# Generate documentation
doc:
	cargo doc --open

# Check formatting
check-format:
	cargo fmt --check

# Run all checks
check: format lint test