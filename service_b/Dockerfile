# Build stage
FROM rust:latest as builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock* ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/service_b /app/service_b

# Expose the port
EXPOSE 3000

# Run the application
CMD ["./service_b"]
