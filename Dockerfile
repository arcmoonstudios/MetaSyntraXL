# Use the official Rust image as the builder
FROM rust:1.63 as builder

# Set the working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Install necessary dependencies (if any)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/metasyntraxl /usr/local/bin/metasyntraxl

# Set environment variables (can be overridden)
ENV RUST_LOG=info

# Expose necessary ports (if applicable)
EXPOSE 8080

# Run the application
CMD ["metasyntraxl"]
