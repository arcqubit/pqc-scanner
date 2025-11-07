# Multi-stage build for minimal distroless container
# Stage 1: Build the WASM binaries
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Node.js for npm scripts
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./
COPY package.json package-lock.json* ./

# Copy source code
COPY src ./src
COPY benches ./benches
COPY tests ./tests

# Build WASM binaries (all targets)
RUN cargo build --release --target wasm32-unknown-unknown
RUN npm install
RUN npm run build:release

# Optimize WASM binaries
RUN npm install -g wasm-opt
RUN wasm-opt -Oz -o pkg/rust_wasm_app_bg.wasm pkg/rust_wasm_app_bg.wasm
RUN wasm-opt -Oz -o pkg-nodejs/rust_wasm_app_bg.wasm pkg-nodejs/rust_wasm_app_bg.wasm
RUN wasm-opt -Oz -o pkg-web/rust_wasm_app_bg.wasm pkg-web/rust_wasm_app_bg.wasm

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
COPY --from=builder /build/package.json /app/
COPY --from=builder /build/README.md /app/

# Set working directory
WORKDIR /app

# Run as non-root user
USER nonroot:nonroot

# Health check (if running as service)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD node -e "require('./pkg-nodejs/rust_wasm_app.js')" || exit 1

# Default command: show version and usage
CMD ["--help"]

# Usage:
# docker run --rm ghcr.io/arcqubit/pqc-scanner:latest
# docker run --rm -v $(pwd):/data ghcr.io/arcqubit/pqc-scanner:latest node -e "const scanner = require('/app/pkg-nodejs/rust_wasm_app.js'); console.log(scanner)"
