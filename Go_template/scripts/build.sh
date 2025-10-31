#!/bin/bash

# Build script for Go_template

# Set the project name
PROJECT_NAME="Go_template"

# Set the output directory
OUTPUT_DIR="./bin"

# Create the output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Build the application
echo "Building $PROJECT_NAME..."
go build -o "$OUTPUT_DIR/$PROJECT_NAME" ./cmd/$PROJECT_NAME

# Check if the build was successful
if [ $? -eq 0 ]; then
    echo "Build successful! Binary is located at $OUTPUT_DIR/$PROJECT_NAME"
else
    echo "Build failed!"
    exit 1
fi