# Use an official Rust runtime as a parent image
FROM rust:1.58 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a slim image for the final container
FROM debian:buster-slim

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/edge-ai-app .

# Expose the port the app runs on
EXPOSE 8080

# Run the binary
CMD ["./edge-ai-app"]
