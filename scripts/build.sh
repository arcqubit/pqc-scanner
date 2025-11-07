#!/bin/bash
# Build script for PhotonIQ PQC Scanner
# Builds Rust library and WASM packages for multiple targets

set -e

echo "🔨 Building PhotonIQ PQC Scanner..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}⚠️  wasm-pack not found. Installing...${NC}"
    cargo install wasm-pack
fi

# Parse arguments
RELEASE_MODE=""
if [ "$1" == "--release" ]; then
    RELEASE_MODE="--release"
    echo "📦 Building in RELEASE mode"
else
    echo "🔧 Building in DEBUG mode"
fi
echo ""

# Build Rust library
echo "1️⃣  Building Rust library..."
cargo build $RELEASE_MODE
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Rust build successful${NC}"
else
    echo -e "${RED}✗ Rust build failed${NC}"
    exit 1
fi
echo ""

# Run tests
echo "2️⃣  Running tests..."
cargo test --quiet
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed${NC}"
else
    echo -e "${RED}✗ Tests failed${NC}"
    exit 1
fi
echo ""

# Build WASM for bundler target
echo "3️⃣  Building WASM (bundler target)..."
wasm-pack build --target bundler --out-dir pkg $RELEASE_MODE
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Bundler build successful${NC}"
else
    echo -e "${RED}✗ Bundler build failed${NC}"
    exit 1
fi

# Build WASM for Node.js target
echo "4️⃣  Building WASM (Node.js target)..."
wasm-pack build --target nodejs --out-dir pkg-nodejs $RELEASE_MODE
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Node.js build successful${NC}"
else
    echo -e "${RED}✗ Node.js build failed${NC}"
    exit 1
fi

# Build WASM for web target
echo "5️⃣  Building WASM (web target)..."
wasm-pack build --target web --out-dir pkg-web $RELEASE_MODE
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Web build successful${NC}"
else
    echo -e "${RED}✗ Web build failed${NC}"
    exit 1
fi
echo ""

# Show WASM sizes
echo "📊 WASM Bundle Sizes:"
echo "-----------------------------------"
for target in pkg pkg-nodejs pkg-web; do
    if [ -d "$target" ]; then
        wasm_file=$(find "$target" -name "*.wasm" | head -1)
        if [ -f "$wasm_file" ]; then
            size=$(ls -lh "$wasm_file" | awk '{print $5}')
            echo "  $target: $size"
        fi
    fi
done
echo ""

# Check size threshold
MAX_SIZE=524288  # 512KB
for target in pkg pkg-nodejs pkg-web; do
    wasm_file=$(find "$target" -name "*.wasm" | head -1)
    if [ -f "$wasm_file" ]; then
        size=$(stat -c%s "$wasm_file" 2>/dev/null || stat -f%z "$wasm_file" 2>/dev/null)
        if [ $size -gt $MAX_SIZE ]; then
            echo -e "${YELLOW}⚠️  Warning: $target WASM bundle exceeds 512KB target${NC}"
        fi
    fi
done

echo ""
echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo "📦 Packages ready:"
echo "  - pkg/           (bundler target)"
echo "  - pkg-nodejs/    (Node.js target)"
echo "  - pkg-web/       (web target)"
echo ""
echo "🚀 Next steps:"
echo "  - Test: npm test"
echo "  - Publish: npm publish"
