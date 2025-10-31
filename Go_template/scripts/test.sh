#!/bin/bash

# Test script for Go_template

# Run unit tests
echo "Running unit tests..."
go test ./...

# Check if the tests were successful
if [ $? -eq 0 ]; then
    echo "All tests passed!"
else
    echo "Some tests failed!"
    exit 1
fi

# Run integration tests (if any)
echo "Running integration tests..."
go test ./tests/integration/...

# Check if the integration tests were successful
if [ $? -eq 0 ]; then
    echo "All integration tests passed!"
else
    echo "Some integration tests failed!"
    exit 1
fi