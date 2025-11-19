.PHONY: help all build build-release test clean install lint format bench wasm wasm-release example geiger udeps scan-samples docker-build docker-build-multiarch docker-build-push docker-push docker-test docker-login docker-clean docker-run

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

# Docker configuration
REGISTRY ?= ghcr.io
IMAGE_NAME ?= arcqubit/pqc-scanner
VERSION ?= $(shell grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)
IMAGE_TAG ?= $(VERSION)
PLATFORMS ?= linux/amd64,linux/arm64

# Docker targets
docker-build:
	@echo "Building Docker image for local platform..."
	docker buildx build \
		--tag $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG) \
		--tag $(REGISTRY)/$(IMAGE_NAME):latest \
		--tag $(REGISTRY)/$(IMAGE_NAME):beta \
		--load \
		.

docker-build-multiarch:
	@echo "Building multi-arch Docker image (no load, use for testing)..."
	docker buildx build \
		--platform $(PLATFORMS) \
		--tag $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG) \
		--tag $(REGISTRY)/$(IMAGE_NAME):latest \
		--tag $(REGISTRY)/$(IMAGE_NAME):beta \
		.

docker-build-push:
	@echo "Building and pushing multi-arch Docker image..."
	@echo "Note: Ensure you are logged in with 'make docker-login' or 'docker login ghcr.io'"
	docker buildx build \
		--platform $(PLATFORMS) \
		--tag $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG) \
		--tag $(REGISTRY)/$(IMAGE_NAME):latest \
		--tag $(REGISTRY)/$(IMAGE_NAME):beta \
		--push \
		.

docker-push:
	@echo "Pushing Docker image to $(REGISTRY)..."
	docker push $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG)
	docker push $(REGISTRY)/$(IMAGE_NAME):latest
	docker push $(REGISTRY)/$(IMAGE_NAME):beta

docker-test:
	@echo "Testing Docker image $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG)..."
	@echo ""
	@echo "Test 1: Container starts and shows help"
	@docker run --rm $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG) --help 2>&1 | grep -q "PQC Scanner" && echo "✓ Help output contains 'PQC Scanner'" || (echo "✗ Help output test failed" && exit 1)
	@echo ""
	@echo "Test 2: Binary is executable and responds"
	@docker run --rm $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG) 2>&1 | grep -q "Usage:" && echo "✓ Default command shows usage" || (echo "✗ Default command test failed" && exit 1)
	@echo ""
	@echo "✓ All Docker image tests passed"

docker-login:
	@echo "Logging in to GitHub Container Registry..."
	@echo "$(GITHUB_TOKEN)" | docker login $(REGISTRY) -u $(GITHUB_USER) --password-stdin

docker-clean:
	@echo "Cleaning Docker images..."
	docker rmi $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG) || true
	docker rmi $(REGISTRY)/$(IMAGE_NAME):latest || true
	docker rmi $(REGISTRY)/$(IMAGE_NAME):beta || true

docker-run:
	@echo "Running Docker container interactively..."
	docker run --rm -it -v $(PWD):/app/workspace $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG)

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
	@echo "Docker:"
	@echo "  make docker-build        - Build for local platform only (fast)"
	@echo "  make docker-build-multiarch - Build multi-arch (amd64/arm64) without loading"
	@echo "  make docker-build-push   - Build multi-arch and push to registry"
	@echo "  make docker-push         - Push existing image to registry"
	@echo "  make docker-test         - Test Docker image locally"
	@echo "  make docker-login        - Login to GitHub Container Registry"
	@echo "  make docker-run          - Run container interactively"
	@echo "  make docker-clean        - Remove local Docker images"
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
