FROM rust:latest AS builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src src

# Copy config files
COPY config config

# Copy models
COPY models models

# Build the application
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd --create-home --shell /bin/bash app
USER app
WORKDIR /home/app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/heterogeneous-computing-architecture .

# Copy configuration files
COPY --from=builder /app/config ./config

# Copy models
COPY --from=builder /app/models ./models

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["./heterogeneous-computing-architecture"]