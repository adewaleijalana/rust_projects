# Stage 1: Builder
# Use a specific Rust image as the base for building
FROM rust:1.85 AS builder
WORKDIR /app

COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runner
# Use a minimal base image for the final application (e.g., debian slim or alpine)
FROM debian:stable-slim
# Install necessary dependencies if any (e.g., for SQL clients)
# RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/user_management ./user_management

# Expose the port your Axum app listens on (default is often 3000)
EXPOSE 8181

# Command to run the application
CMD ["./user_management"]
