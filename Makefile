.PHONY: help all build build-release test clean install lint format bench wasm wasm-release example geiger udeps scan-samples

# Default target - show help
.DEFAULT_GOAL := help

# Build and test
all: build test

# Development build
build:
	cargo build

# Release build
build-release:
	cargo build --release

# Run all tests
test:
	cargo test --verbose

# Run integration tests
test-integration:
	cargo test --test integration_tests

# Clean build artifacts
clean:
	cargo clean
	rm -rf pkg pkg-nodejs pkg-web
	rm -f *.json oscal-*.json sc13-*.json

# Install dependencies
install:
	rustup component add rustfmt clippy
	cargo install wasm-pack || true

# Lint code
lint:
	cargo clippy -- -D warnings

# Format code
format:
	cargo fmt

# Check formatting
format-check:
	cargo fmt -- --check

# Run benchmarks
bench:
	cargo bench

# Unsafe code detection
geiger:
	cargo geiger --all-features --all-targets --exclude-tests --deny=warn

# Unused dependencies detection
udeps:
	cargo +nightly udeps --all-targets --all-features

# Build WASM (all targets)
wasm: install
	wasm-pack build --target bundler --out-dir pkg
	wasm-pack build --target nodejs --out-dir pkg-nodejs
	wasm-pack build --target web --out-dir pkg-web

# Build WASM in release mode
wasm-release: install
	wasm-pack build --target bundler --out-dir pkg --release
	wasm-pack build --target nodejs --out-dir pkg-nodejs --release
	wasm-pack build --target web --out-dir pkg-web --release

# Check WASM size
wasm-size: wasm-release
	@echo "WASM Bundle Sizes:"
	@ls -lh pkg/*.wasm pkg-nodejs/*.wasm pkg-web/*.wasm | awk '{print $$9, $$5}'

# Run compliance report example
example:
	cargo run --example generate_compliance_report

# Scan all sample repositories
scan-samples:
	@echo "Scanning all sample repositories..."
	@./scripts/scan-all-samples.sh

# CI pipeline
ci: lint format-check test bench

# Full release build (Rust + WASM)
release: build-release wasm-release
	@echo "Release build complete"
	@echo "Cargo: target/release/"
	@echo "WASM: pkg/, pkg-nodejs/, pkg-web/"

# Development workflow
dev: format lint test
	@echo "Development checks passed"

# Help
help:
	@echo "PQC Scanner - Build Targets"
	@echo "===================================="
	@echo "Development:"
	@echo "  make build          - Build debug version"
	@echo "  make test           - Run all tests"
	@echo "  make dev            - Format, lint, and test"
	@echo "  make example        - Run compliance report example"
	@echo ""
	@echo "Release:"
	@echo "  make build-release  - Build optimized Rust binary"
	@echo "  make wasm-release   - Build optimized WASM packages"
	@echo "  make release        - Full release build"
	@echo ""
	@echo "Quality:"
	@echo "  make lint           - Run clippy linter"
	@echo "  make format         - Format code"
	@echo "  make format-check   - Check formatting"
	@echo "  make bench          - Run benchmarks"
	@echo "  make geiger         - Detect unsafe code usage"
	@echo "  make udeps          - Detect unused dependencies"
	@echo ""
	@echo "WASM:"
	@echo "  make wasm           - Build WASM (debug)"
	@echo "  make wasm-release   - Build WASM (optimized)"
	@echo "  make wasm-size      - Show WASM bundle sizes"
	@echo ""
	@echo "Sample Repositories:"
	@echo "  make scan-samples   - Scan all sample vulnerable repositories"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make install        - Install build tools"
	@echo "  make ci             - Run CI checks"
