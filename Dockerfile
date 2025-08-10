
# Use the latest Rust version (1.83) that supports all modern dependencies
FROM rust:1.83-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {println!(\"placeholder\");}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove ALL build artifacts including the binary
RUN rm -rf src target/release/deps/solana_usdc_indexer* target/release/indexer target/release/build target/release/.fingerprint

# Copy actual source code
COPY src ./src

# Build the final application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/indexer /app/indexer

# Change ownership
RUN chown -R appuser:appuser /app

# Switch to app user
USER appuser

# Expose port (if needed for future web interface)
EXPOSE 8080

# Default command
CMD ["./indexer"]

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/indexer /app/indexer

# Change ownership
RUN chown -R appuser:appuser /app

# Switch to app user
USER appuser

# Expose port (if needed for future web interface)
EXPOSE 8080

# Default command
CMD ["./indexer"]