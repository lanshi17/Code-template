#!/bin/bash

# Run script for Go_template

# Set the project name
PROJECT_NAME="Go_template"

# Set the binary path
BINARY_PATH="./bin/$PROJECT_NAME"

# Check if the binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Binary not found at $BINARY_PATH. Building the project..."
    ./scripts/build.sh
    
    # Check if the build was successful
    if [ $? -ne 0 ]; then
        echo "Failed to build the project!"
        exit 1
    fi
fi

# Run the application
echo "Running $PROJECT_NAME..."
"$BINARY_PATH"