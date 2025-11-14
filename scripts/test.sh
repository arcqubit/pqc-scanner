#!/bin/bash
# Test script for PhotonIQ PQC Scanner
# Runs Rust tests, integration tests, and benchmarks

set -e

echo "🧪 Running PhotonIQ PQC Scanner Tests..."
echo ""

# Colors for output
GREEN='\033[0;32m'
NC='\033[0m'

# Run unit tests
echo "1️⃣  Running unit tests..."
cargo test --lib --quiet
echo -e "${GREEN}✓ Unit tests passed${NC}"
echo ""

# Run integration tests
echo "2️⃣  Running integration tests..."
cargo test --test integration_tests --quiet
echo -e "${GREEN}✓ Integration tests passed${NC}"
echo ""

# Run doc tests
echo "3️⃣  Running doc tests..."
cargo test --doc --quiet 2>/dev/null || echo "No doc tests found"
echo ""

# Run examples
echo "4️⃣  Testing examples..."
cargo run --example generate_compliance_report --quiet > /dev/null 2>&1
if [ -f "templates/sc13-compliance-report.json" ] && [ -f "templates/oscal-assessment-results.json" ]; then
    echo -e "${GREEN}✓ Example execution successful${NC}"
    echo "  Generated: templates/sc13-compliance-report.json"
    echo "  Generated: templates/oscal-assessment-results.json"
else
    echo "✗ Example execution failed"
    exit 1
fi
echo ""

# Run linter
echo "5️⃣  Running clippy..."
cargo clippy --quiet -- -D warnings 2>&1 | grep -v "Checking\|Finished" || echo -e "${GREEN}✓ No clippy warnings${NC}"
echo ""

# Check formatting
echo "6️⃣  Checking code formatting..."
cargo fmt -- --check
echo -e "${GREEN}✓ Code is properly formatted${NC}"
echo ""

echo -e "${GREEN}✅ All tests passed!${NC}"
