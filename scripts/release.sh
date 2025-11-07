#!/bin/bash
# Release script for PhotonIQ PQC Scanner
# Prepares and publishes a new release

set -e

echo "🚀 PhotonIQ PQC Scanner Release Process"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check if version is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 1.0.0"
    exit 1
fi

VERSION=$1
echo "📦 Preparing release v${VERSION}"
echo ""

# Validate version format
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Expected: X.Y.Z (e.g., 1.0.0)"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: Uncommitted changes detected${NC}"
    echo "Please commit or stash your changes before releasing"
    exit 1
fi

# Update version in Cargo.toml
echo "1️⃣  Updating Cargo.toml version..."
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
rm -f Cargo.toml.bak
echo -e "${GREEN}✓ Version updated to ${VERSION}${NC}"
echo ""

# Update version in package.json
echo "2️⃣  Updating package.json version..."
if [ -f package.json ]; then
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" package.json
    rm -f package.json.bak
    echo -e "${GREEN}✓ package.json updated${NC}"
fi
echo ""

# Run tests
echo "3️⃣  Running full test suite..."
./scripts/test.sh
echo ""

# Build release
echo "4️⃣  Building release artifacts..."
./scripts/build.sh --release
echo ""

# Generate changelog
echo "5️⃣  Generating changelog..."
if command -v git-cliff &> /dev/null; then
    git-cliff --tag v${VERSION} > CHANGELOG.md
    echo -e "${GREEN}✓ Changelog generated${NC}"
else
    echo -e "${YELLOW}⚠️  git-cliff not found, skipping changelog${NC}"
fi
echo ""

# Git commit
echo "6️⃣  Creating git commit..."
git add Cargo.toml Cargo.lock package.json CHANGELOG.md 2>/dev/null || true
git commit -m "chore: Release v${VERSION}

- Updated version to ${VERSION}
- Built WASM packages
- Generated changelog

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
echo -e "${GREEN}✓ Commit created${NC}"
echo ""

# Git tag
echo "7️⃣  Creating git tag..."
git tag -a "v${VERSION}" -m "Release v${VERSION}"
echo -e "${GREEN}✓ Tag created: v${VERSION}${NC}"
echo ""

# Summary
echo ""
echo -e "${GREEN}✅ Release v${VERSION} prepared!${NC}"
echo ""
echo "📋 Next steps:"
echo "  1. Review the changes:"
echo "     git show HEAD"
echo ""
echo "  2. Push to remote:"
echo "     git push origin main"
echo "     git push origin v${VERSION}"
echo ""
echo "  3. Publish to npm:"
echo "     npm publish"
echo ""
echo "  4. Create GitHub release:"
echo "     gh release create v${VERSION} --generate-notes"
