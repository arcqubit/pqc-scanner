# Multi-stage build for minimal distroless container
# Stage 1: Build the WASM binaries
FROM rust:1.83-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Set working directory
WORKDIR /build

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY benches ./benches
COPY tests ./tests

# Build WASM binaries (all targets)
RUN wasm-pack build --target bundler --out-dir pkg --release
RUN wasm-pack build --target nodejs --out-dir pkg-nodejs --release
RUN wasm-pack build --target web --out-dir pkg-web --release

# WASM binaries are already optimized by wasm-pack --release
# wasm-pack applies wasm-opt optimizations automatically
# No additional optimization step needed

# Stage 2: Runtime image with distroless Node.js
FROM gcr.io/distroless/nodejs20-debian12:nonroot

LABEL org.opencontainers.image.title="ArcQubit PQC Scanner"
LABEL org.opencontainers.image.description="Quantum-safe cryptography auditor with WASM"
LABEL org.opencontainers.image.vendor="ArcQubit Team"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/arcqubit/pqc-scanner"

# Copy built WASM artifacts from builder
COPY --from=builder /build/pkg /app/pkg
COPY --from=builder /build/pkg-nodejs /app/pkg-nodejs
COPY --from=builder /build/pkg-web /app/pkg-web
COPY --from=builder /build/README.md /app/

# Set working directory
WORKDIR /app

# Run as non-root user
USER nonroot:nonroot

# Health check (if running as service)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD node -e "require('./pkg-nodejs/pqc_scanner.js')" || exit 1

# Default command: show version and usage
CMD ["--help"]

# Usage:
# docker run --rm ghcr.io/arcqubit/pqc-scanner:latest
# docker run --rm -v $(pwd):/data ghcr.io/arcqubit/pqc-scanner:latest node -e "const scanner = require('/app/pkg-nodejs/pqc_scanner.js'); console.log(scanner)"
