# Multi-stage Dockerfile for PQC Scanner
# Builds CLI binary for quantum-vulnerable cryptography scanning
# Optimized for size (<50MB), security (Alpine + non-root), and performance

# ============================================================================
# Build Arguments
# ============================================================================
ARG ALPINE_VERSION=3.20

# ============================================================================
# Stage 1: Rust CLI Builder (Alpine-based for minimal size)
# ============================================================================
# Using nightly for edition2024 support
FROM rustlang/rust:nightly-alpine AS rust-builder

# Install build dependencies for static linking
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /build

# Layer caching: Copy dependency manifests first
COPY Cargo.toml Cargo.lock ./

# Copy source and data
COPY src/ ./src/
COPY data/ ./data/
COPY benches/ ./benches/
COPY examples/ ./examples/
COPY tests/ ./tests/

# Build optimized CLI binary with static linking
RUN cargo build --release --bin pqc-scanner && \
    strip /build/target/release/pqc-scanner

# ============================================================================
# Stage 2: Runtime Image (Minimal Alpine)
# ============================================================================
FROM alpine:${ALPINE_VERSION}

# Install minimal runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    && addgroup -g 1000 pqc \
    && adduser -D -u 1000 -G pqc pqc

WORKDIR /app

# Copy CLI binary from builder
COPY --from=rust-builder /build/target/release/pqc-scanner /usr/local/bin/pqc-scanner

# Copy algorithm databases (required for detection)
COPY --from=rust-builder /build/data/ /app/data/

# Copy documentation
COPY README.md LICENSE /app/

# Create workspace and reports directories
RUN mkdir -p /app/workspace /app/reports && \
    chown -R pqc:pqc /app

# Switch to non-root user for security
USER pqc

# Set working directory for scanning operations
WORKDIR /app/workspace

# Health check: Verify CLI binary works
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD pqc-scanner --version || exit 1

# Default entrypoint: CLI binary
ENTRYPOINT ["pqc-scanner"]
CMD ["--help"]

# ============================================================================
# Container Labels (OCI standard)
# ============================================================================
LABEL org.opencontainers.image.title="PQC Scanner"
LABEL org.opencontainers.image.description="Quantum-safe cryptography auditor for detecting vulnerable algorithms in source code. Supports CLI and WASM for multi-platform deployment."
LABEL org.opencontainers.image.vendor="ArcQubit Team"
LABEL org.opencontainers.image.url="https://github.com/arcqubit/pqc-scanner"
LABEL org.opencontainers.image.source="https://github.com/arcqubit/pqc-scanner"
LABEL org.opencontainers.image.documentation="https://github.com/arcqubit/pqc-scanner/blob/main/README.md"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.version="2025.11.18"

# ============================================================================
# Usage Examples
# ============================================================================
# Build:
#   docker build -t pqc-scanner .
#
# Run CLI (default):
#   docker run --rm pqc-scanner --version
#   docker run --rm -v $(pwd):/app/workspace pqc-scanner scan .
#
# Use WASM (Node.js):
#   docker run --rm pqc-scanner:latest node -e "const pqc = require('/app/pkg-nodejs/pqc_scanner.js'); console.log(pqc)"
#
# Interactive shell:
#   docker run --rm -it --entrypoint sh pqc-scanner
#
# Scan with output:
#   docker run --rm -v $(pwd):/app/workspace -v $(pwd)/reports:/app/reports pqc-scanner scan . --output /app/reports/report.json
# ============================================================================
