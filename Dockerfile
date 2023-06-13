# Use the official Rust base image
FROM rust:1.67.1-slim

# Set the working directory inside the container
WORKDIR /LMDB-KV-Store

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src ./src

COPY data ./data

# Build the application
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

# Set the entry point to run the compiled application
CMD ["cargo", "run", "--release"]
