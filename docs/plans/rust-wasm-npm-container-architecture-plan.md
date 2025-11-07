# Rust WASM → TypeScript → npm → Container → GitHub Action
## Complete Multi-Platform Deployment Architecture Plan

**Status**: 📋 PLANNING PHASE
**Created**: January 2025
**Purpose**: Design and implement a Rust-based application compiled to WebAssembly, wrapped in TypeScript, published to npm, containerized with distroless, and packaged as a GitHub Action

---

## 📋 Executive Summary

### Objective
Create a production-ready, multi-platform deployment pipeline for a Rust application that can be consumed in multiple ways:
1. **npm package** - JavaScript/TypeScript developers
2. **Standalone container** - Cloud-native deployments
3. **GitHub Action** - CI/CD automation

### Architecture Overview
```
┌─────────────────────────────────────────────────────────────────┐
│                     Development Pipeline                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐      ┌──────────────┐      ┌──────────────┐  │
│  │  Rust Code   │ ──→  │ WASM Module  │ ──→  │  TypeScript  │  │
│  │  (src/*.rs)  │      │  (.wasm)     │      │   Wrapper    │  │
│  └──────────────┘      └──────────────┘      └──────────────┘  │
│         │                      │                      │         │
│         │                      │                      ▼         │
│         │                      │              ┌──────────────┐  │
│         │                      │              │ npm Package  │  │
│         │                      │              │  published   │  │
│         │                      │              └──────────────┘  │
│         │                      │                               │
│         ▼                      ▼                               │
│  ┌──────────────┐      ┌──────────────┐                       │
│  │   Native     │      │  Distroless  │                       │
│  │   Binary     │ ──→  │  Container   │                       │
│  │ (for Docker) │      │    Image     │                       │
│  └──────────────┘      └──────────────┘                       │
│         │                      │                               │
│         │                      ▼                               │
│         │              ┌──────────────┐                       │
│         └─────────────→│ GitHub Action│                       │
│                        │   Package    │                       │
│                        └──────────────┘                       │
└─────────────────────────────────────────────────────────────────┘

Outputs:
  • @your-org/rust-wasm-app (npm)
  • ghcr.io/your-org/rust-wasm-app (container)
  • your-org/rust-wasm-action (GitHub Action)
```

### Success Criteria
- [ ] Rust code compiles to WASM successfully
- [ ] TypeScript bindings provide type-safe API
- [ ] npm package installable and functional
- [ ] Container runs with minimal attack surface (distroless)
- [ ] GitHub Action integrates seamlessly in workflows
- [ ] CI/CD pipeline automates all publishing steps
- [ ] Documentation covers all usage patterns

---

## 🎯 Requirements Analysis

### Functional Requirements

#### FR1: Rust Core Application
- **Description**: Core business logic written in Rust
- **Features**:
  - Quantum-safe crypto detection (reuse existing audit logic)
  - File parsing and analysis
  - JSON output generation
- **Constraints**: Must be WASM-compatible (no OS-specific syscalls)
- **Priority**: HIGH

#### FR2: WASM Compilation
- **Description**: Compile Rust to WebAssembly for browser/Node.js
- **Tools**: wasm-pack, wasm-bindgen
- **Target**: bundler, nodejs, web
- **Output**: .wasm binary + JS glue code
- **Priority**: HIGH

#### FR3: TypeScript Wrapper
- **Description**: Type-safe TypeScript API around WASM module
- **Features**:
  - Promise-based async API
  - Type definitions (.d.ts)
  - Error handling
  - Browser and Node.js support
- **Priority**: HIGH

#### FR4: npm Package
- **Description**: Publishable npm package with WASM bundled
- **Features**:
  - Proper package.json with exports
  - README with examples
  - Version management
  - TypeScript declarations
- **Priority**: HIGH

#### FR5: Distroless Container
- **Description**: Minimal container with Rust native binary
- **Base Image**: gcr.io/distroless/cc-debian12
- **Features**:
  - No shell, package manager, or unnecessary tools
  - Minimal attack surface
  - Multi-stage build
- **Priority**: MEDIUM

#### FR6: GitHub Action
- **Description**: Reusable action for CI/CD workflows
- **Features**:
  - Input parameters
  - Output results
  - Error reporting
  - Uses container or npm package internally
- **Priority**: MEDIUM

### Non-Functional Requirements

#### NFR1: Performance
- **WASM**: Fast execution (near-native performance)
- **Container**: Quick startup (< 1s)
- **npm package**: Small bundle size (< 2MB)
- **Target**: 95% of native Rust performance

#### NFR2: Security
- **Container**: No unnecessary tools or shells
- **Dependencies**: Minimal, audited dependencies
- **Secrets**: No hardcoded credentials
- **SBOM**: Generate Software Bill of Materials

#### NFR3: Compatibility
- **Node.js**: >= 18.x
- **Browsers**: Modern browsers with WASM support
- **Container**: OCI-compliant, runs on Docker/Podman/K8s
- **GitHub Actions**: Compatible with ubuntu-latest, macos-latest

#### NFR4: Maintainability
- **CI/CD**: Automated testing and publishing
- **Versioning**: Semantic versioning
- **Documentation**: Comprehensive guides
- **Monitoring**: Build and runtime metrics

---

## 🏗️ Detailed Architecture

### 1. Rust Application Structure

```
rust-wasm-app/
├── Cargo.toml              # Rust dependencies
├── Cargo.lock
├── src/
│   ├── lib.rs              # WASM entry point
│   ├── audit.rs            # Core audit logic
│   ├── parser.rs           # File parsing
│   ├── detector.rs         # Crypto detection
│   └── types.rs            # Shared types
├── tests/
│   ├── integration.rs      # Integration tests
│   └── fixtures/           # Test data
└── benches/
    └── benchmarks.rs       # Performance tests
```

#### Key Dependencies (Cargo.toml)
```toml
[package]
name = "rust-wasm-app"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
serde_json = "1.0"
regex = "1.10"
console_error_panic_hook = "0.1"

[dev-dependencies]
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = "z"         # Optimize for size
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
strip = true            # Remove debug symbols
```

#### lib.rs Structure
```rust
use wasm_bindgen::prelude::*;

// Entry point for WASM initialization
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

// Main audit function exposed to JS
#[wasm_bindgen]
pub fn audit_code(source: &str, language: &str) -> Result<JsValue, JsValue> {
    let result = audit::analyze(source, language)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub struct AuditResult {
    vulnerabilities: Vec<Vulnerability>,
    risk_score: u32,
    recommendations: Vec<String>,
}

#[wasm_bindgen]
impl AuditResult {
    pub fn get_vulnerabilities(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.vulnerabilities).unwrap()
    }

    pub fn risk_score(&self) -> u32 {
        self.risk_score
    }
}
```

### 2. WASM Build Configuration

#### Build Script (build-wasm.sh)
```bash
#!/bin/bash
set -e

# Build for different targets
echo "Building WASM for bundler..."
wasm-pack build --target bundler --out-dir pkg/bundler

echo "Building WASM for Node.js..."
wasm-pack build --target nodejs --out-dir pkg/nodejs

echo "Building WASM for web..."
wasm-pack build --target web --out-dir pkg/web

echo "Optimizing WASM with wasm-opt..."
if command -v wasm-opt >/dev/null 2>&1; then
    for pkg in pkg/*/; do
        wasm-opt -Oz "$pkg"/*.wasm -o "$pkg"/*.wasm
    done
fi

echo "WASM build complete!"
```

#### wasm-pack Configuration
```json
{
  "name": "@your-org/rust-wasm-app",
  "version": "0.1.0",
  "files": [
    "pkg/bundler",
    "pkg/nodejs",
    "pkg/web"
  ]
}
```

### 3. TypeScript Wrapper Structure

```
typescript-wrapper/
├── package.json
├── tsconfig.json
├── src/
│   ├── index.ts            # Main entry point
│   ├── wasm-loader.ts      # WASM initialization
│   ├── types.ts            # TypeScript types
│   └── api.ts              # High-level API
├── dist/                   # Compiled output
│   ├── index.js
│   ├── index.d.ts
│   └── wasm/               # WASM files
└── tests/
    └── api.test.ts         # TypeScript tests
```

#### package.json
```json
{
  "name": "@your-org/rust-wasm-app",
  "version": "0.1.0",
  "description": "Quantum-safe cryptography audit tool (Rust + WASM + TypeScript)",
  "main": "dist/index.js",
  "module": "dist/index.mjs",
  "types": "dist/index.d.ts",
  "exports": {
    ".": {
      "import": "./dist/index.mjs",
      "require": "./dist/index.js",
      "types": "./dist/index.d.ts"
    },
    "./wasm": {
      "import": "./dist/wasm/rust_wasm_app_bg.wasm",
      "require": "./dist/wasm/rust_wasm_app_bg.wasm"
    }
  },
  "files": [
    "dist",
    "README.md",
    "LICENSE"
  ],
  "scripts": {
    "build:rust": "./build-wasm.sh",
    "build:ts": "tsup src/index.ts --format cjs,esm --dts",
    "build": "npm run build:rust && npm run build:ts",
    "test": "vitest run",
    "prepublishOnly": "npm run build && npm test"
  },
  "keywords": [
    "quantum-safe",
    "cryptography",
    "audit",
    "wasm",
    "rust",
    "security"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/your-org/rust-wasm-app"
  },
  "engines": {
    "node": ">=18.0.0"
  },
  "dependencies": {},
  "devDependencies": {
    "@types/node": "^20.0.0",
    "tsup": "^8.0.0",
    "typescript": "^5.3.0",
    "vitest": "^1.0.0"
  }
}
```

#### src/index.ts
```typescript
import init, { audit_code, AuditResult } from '../pkg/nodejs/rust_wasm_app';

let wasmInitialized = false;

/**
 * Initialize the WASM module
 */
export async function initialize(): Promise<void> {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

/**
 * Audit result interface
 */
export interface Vulnerability {
  type: string;
  severity: 'critical' | 'high' | 'medium' | 'low';
  line: number;
  column: number;
  message: string;
  recommendation: string;
}

export interface AuditResultData {
  vulnerabilities: Vulnerability[];
  riskScore: number;
  recommendations: string[];
}

/**
 * Main audit API
 */
export class QuantumSafeAuditor {
  private initialized: boolean = false;

  async init(): Promise<void> {
    if (!this.initialized) {
      await initialize();
      this.initialized = true;
    }
  }

  /**
   * Analyze source code for quantum-vulnerable cryptography
   */
  async auditCode(
    source: string,
    language: 'rust' | 'javascript' | 'typescript' | 'python' | 'java' | 'go'
  ): Promise<AuditResultData> {
    await this.init();

    try {
      const result = audit_code(source, language);
      return this.parseResult(result);
    } catch (error) {
      throw new Error(`Audit failed: ${error}`);
    }
  }

  /**
   * Audit a file path (Node.js only)
   */
  async auditFile(filePath: string): Promise<AuditResultData> {
    if (typeof window !== 'undefined') {
      throw new Error('auditFile is only available in Node.js');
    }

    const fs = await import('fs/promises');
    const path = await import('path');

    const source = await fs.readFile(filePath, 'utf-8');
    const ext = path.extname(filePath).slice(1);
    const language = this.getLanguageFromExtension(ext);

    return this.auditCode(source, language);
  }

  /**
   * Batch audit multiple files
   */
  async auditFiles(filePaths: string[]): Promise<Map<string, AuditResultData>> {
    const results = new Map<string, AuditResultData>();

    await Promise.all(
      filePaths.map(async (filePath) => {
        try {
          const result = await this.auditFile(filePath);
          results.set(filePath, result);
        } catch (error) {
          console.error(`Failed to audit ${filePath}:`, error);
        }
      })
    );

    return results;
  }

  private parseResult(result: AuditResult): AuditResultData {
    return {
      vulnerabilities: result.get_vulnerabilities(),
      riskScore: result.risk_score(),
      recommendations: [],
    };
  }

  private getLanguageFromExtension(ext: string): string {
    const langMap: Record<string, string> = {
      rs: 'rust',
      js: 'javascript',
      ts: 'typescript',
      py: 'python',
      java: 'java',
      go: 'go',
    };
    return langMap[ext] || 'javascript';
  }
}

// Export for convenience
export const auditor = new QuantumSafeAuditor();

// Export WASM init function
export { initialize as initWasm };
```

#### tsconfig.json
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "lib": ["ES2020"],
    "declaration": true,
    "declarationMap": true,
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "allowSyntheticDefaultImports": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "tests"]
}
```

### 4. Distroless Container Configuration

```
docker/
├── Dockerfile              # Multi-stage build
├── .dockerignore
└── entrypoint.sh
```

#### Dockerfile (Multi-stage Distroless)
```dockerfile
# Stage 1: Rust builder
FROM rust:1.75-slim AS rust-builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build release binary (native, not WASM)
RUN cargo build --release --bin rust-wasm-app-cli

# Stage 2: Distroless runtime
FROM gcr.io/distroless/cc-debian12:nonroot

# Copy binary from builder
COPY --from=rust-builder /build/target/release/rust-wasm-app-cli /app/audit

# Set working directory
WORKDIR /workspace

# Use non-root user (defined in distroless)
USER nonroot:nonroot

# Entrypoint
ENTRYPOINT ["/app/audit"]
CMD ["--help"]

# Metadata
LABEL org.opencontainers.image.title="Rust WASM Quantum Audit"
LABEL org.opencontainers.image.description="Quantum-safe cryptography audit tool"
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.source="https://github.com/your-org/rust-wasm-app"
```

#### .dockerignore
```
target/
node_modules/
pkg/
dist/
.git/
*.wasm
*.log
```

#### Build and Run Scripts
```bash
# build-container.sh
#!/bin/bash
set -e

IMAGE_NAME="ghcr.io/your-org/rust-wasm-app"
VERSION="${1:-latest}"

echo "Building distroless container: $IMAGE_NAME:$VERSION"

docker build \
  -f docker/Dockerfile \
  -t "$IMAGE_NAME:$VERSION" \
  -t "$IMAGE_NAME:latest" \
  .

echo "Container built successfully!"
echo "Run: docker run --rm -v \$(pwd):/workspace $IMAGE_NAME:$VERSION audit ./src"
```

### 5. GitHub Action Configuration

```
action/
├── action.yml              # GitHub Action definition
├── Dockerfile              # Container action
└── README.md               # Action documentation
```

#### action.yml
```yaml
name: 'Quantum-Safe Crypto Audit'
description: 'Audit code for quantum-vulnerable cryptographic algorithms'
author: 'Your Organization'

branding:
  icon: 'shield'
  color: 'blue'

inputs:
  path:
    description: 'Path to source code to audit'
    required: false
    default: '.'

  language:
    description: 'Programming language (auto-detect if not specified)'
    required: false
    default: 'auto'

  fail-on-critical:
    description: 'Fail the action if critical vulnerabilities found'
    required: false
    default: 'true'

  output-format:
    description: 'Output format (json, sarif, text)'
    required: false
    default: 'text'

  output-file:
    description: 'File to write results to'
    required: false
    default: ''

outputs:
  vulnerabilities:
    description: 'Number of vulnerabilities found'

  risk-score:
    description: 'Overall risk score (0-100)'

  report:
    description: 'JSON report of findings'

runs:
  using: 'docker'
  image: 'Dockerfile'
  args:
    - ${{ inputs.path }}
    - ${{ inputs.language }}
    - ${{ inputs.fail-on-critical }}
    - ${{ inputs.output-format }}
    - ${{ inputs.output-file }}
```

#### GitHub Action Dockerfile
```dockerfile
FROM ghcr.io/your-org/rust-wasm-app:latest

# Add GitHub Action wrapper script
COPY action-wrapper.sh /action-wrapper.sh
RUN chmod +x /action-wrapper.sh

ENTRYPOINT ["/action-wrapper.sh"]
```

#### action-wrapper.sh
```bash
#!/bin/sh
set -e

PATH_TO_AUDIT="${1:-.}"
LANGUAGE="${2:-auto}"
FAIL_ON_CRITICAL="${3:-true}"
OUTPUT_FORMAT="${4:-text}"
OUTPUT_FILE="${5:-}"

echo "🔍 Running Quantum-Safe Crypto Audit..."
echo "Path: $PATH_TO_AUDIT"
echo "Language: $LANGUAGE"

# Run audit
RESULT=$(/app/audit \
  --path "$PATH_TO_AUDIT" \
  --language "$LANGUAGE" \
  --format "$OUTPUT_FORMAT" \
  2>&1)

EXIT_CODE=$?

# Parse results
VULN_COUNT=$(echo "$RESULT" | grep -o "vulnerabilities: [0-9]*" | grep -o "[0-9]*" || echo "0")
RISK_SCORE=$(echo "$RESULT" | grep -o "risk-score: [0-9]*" | grep -o "[0-9]*" || echo "0")

# Set outputs
echo "vulnerabilities=$VULN_COUNT" >> "$GITHUB_OUTPUT"
echo "risk-score=$RISK_SCORE" >> "$GITHUB_OUTPUT"

# Write to file if specified
if [ -n "$OUTPUT_FILE" ]; then
  echo "$RESULT" > "$OUTPUT_FILE"
  echo "📄 Report written to $OUTPUT_FILE"
fi

# Print summary
echo ""
echo "📊 Audit Summary:"
echo "  Vulnerabilities: $VULN_COUNT"
echo "  Risk Score: $RISK_SCORE"

# Fail if critical and requested
if [ "$FAIL_ON_CRITICAL" = "true" ] && [ "$EXIT_CODE" -ne 0 ]; then
  echo "❌ Critical vulnerabilities found!"
  exit 1
fi

echo "✅ Audit complete!"
exit 0
```

### 6. Example GitHub Workflow Usage

```yaml
# .github/workflows/security-audit.yml
name: Quantum-Safe Crypto Audit

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Run Quantum-Safe Audit
        uses: your-org/rust-wasm-action@v1
        with:
          path: './src'
          fail-on-critical: true
          output-format: 'sarif'
          output-file: 'audit-results.sarif'

      - name: Upload SARIF results
        uses: github/codeql-action/upload-sarif@v3
        if: always()
        with:
          sarif_file: audit-results.sarif
```

---

## 🚀 CI/CD Pipeline Architecture

### GitHub Actions Workflow

```
.github/workflows/
├── build-and-test.yml      # Build, test, verify
├── publish-npm.yml         # Publish to npm
├── publish-container.yml   # Publish to GHCR
├── publish-action.yml      # Publish GitHub Action
└── release.yml             # Coordinated release
```

#### build-and-test.yml
```yaml
name: Build and Test

on:
  push:
    branches: [ main, develop ]
  pull_request:

jobs:
  test-rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Run Rust tests
        run: cargo test --all-features

      - name: Run Rust benchmarks
        run: cargo bench

  build-wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Build WASM
        run: ./build-wasm.sh

      - name: Upload WASM artifacts
        uses: actions/upload-artifact@v4
        with:
          name: wasm-build
          path: pkg/

  test-typescript:
    needs: build-wasm
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Download WASM artifacts
        uses: actions/download-artifact@v4
        with:
          name: wasm-build
          path: pkg/

      - name: Install dependencies
        run: npm ci

      - name: Run TypeScript tests
        run: npm test

      - name: Build TypeScript
        run: npm run build

  build-container:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build distroless container
        run: docker build -f docker/Dockerfile -t test-image .

      - name: Test container
        run: docker run --rm test-image --version
```

#### publish-npm.yml
```yaml
name: Publish to npm

on:
  release:
    types: [published]

jobs:
  publish:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          registry-url: 'https://registry.npmjs.org'

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Build package
        run: npm run build

      - name: Run tests
        run: npm test

      - name: Publish to npm
        run: npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

#### publish-container.yml
```yaml
name: Publish Container

on:
  release:
    types: [published]
  workflow_dispatch:

jobs:
  publish:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - uses: actions/checkout@v4

      - name: Log in to GHCR
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Extract metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ghcr.io/${{ github.repository }}
          tags: |
            type=semver,pattern={{version}}
            type=semver,pattern={{major}}.{{minor}}
            type=sha

      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          file: docker/Dockerfile
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
```

---

## 📊 Project Structure (Complete)

```
rust-wasm-app/
├── Cargo.toml
├── Cargo.lock
├── package.json
├── tsconfig.json
├── README.md
├── LICENSE
├── .gitignore
├── .dockerignore
│
├── src/                    # Rust source
│   ├── lib.rs              # WASM entry point
│   ├── main.rs             # CLI binary
│   ├── audit.rs
│   ├── parser.rs
│   ├── detector.rs
│   └── types.rs
│
├── typescript/             # TypeScript wrapper
│   ├── src/
│   │   ├── index.ts
│   │   ├── wasm-loader.ts
│   │   ├── types.ts
│   │   └── api.ts
│   └── tests/
│       └── api.test.ts
│
├── pkg/                    # WASM output (gitignored)
│   ├── bundler/
│   ├── nodejs/
│   └── web/
│
├── dist/                   # TypeScript output (gitignored)
│   ├── index.js
│   ├── index.mjs
│   └── index.d.ts
│
├── docker/
│   ├── Dockerfile
│   └── .dockerignore
│
├── action/                 # GitHub Action
│   ├── action.yml
│   ├── Dockerfile
│   ├── action-wrapper.sh
│   └── README.md
│
├── .github/
│   └── workflows/
│       ├── build-and-test.yml
│       ├── publish-npm.yml
│       ├── publish-container.yml
│       └── release.yml
│
├── scripts/
│   ├── build-wasm.sh
│   ├── build-container.sh
│   └── test-all.sh
│
├── docs/
│   ├── API.md
│   ├── CONTRIBUTING.md
│   ├── USAGE.md
│   └── plans/
│       └── rust-wasm-npm-container-architecture-plan.md (this file)
│
└── examples/
    ├── nodejs/
    │   └── example.js
    ├── browser/
    │   └── index.html
    └── github-action/
        └── workflow.yml
```

---

## 📝 Implementation Phases

### Phase 1: Rust Core Development (Week 1)
**Objective**: Build and test Rust core functionality

**Tasks**:
- [ ] Initialize Cargo project with proper structure
- [ ] Implement core audit logic (`audit.rs`)
- [ ] Create file parser (`parser.rs`)
- [ ] Build crypto detector (`detector.rs`)
- [ ] Define shared types (`types.rs`)
- [ ] Write comprehensive Rust tests
- [ ] Add benchmarks for performance validation
- [ ] Document Rust API

**Deliverables**:
- Working Rust library (non-WASM)
- Test coverage > 80%
- Benchmark results

**Success Criteria**:
- ✅ All tests pass
- ✅ Benchmarks show acceptable performance
- ✅ Code passes clippy lints

### Phase 2: WASM Compilation (Week 2)
**Objective**: Compile Rust to WebAssembly

**Tasks**:
- [ ] Add wasm-bindgen dependencies
- [ ] Create WASM-compatible lib.rs
- [ ] Configure wasm-pack for multiple targets
- [ ] Build bundler, nodejs, and web targets
- [ ] Optimize WASM with wasm-opt
- [ ] Write WASM-specific tests
- [ ] Measure WASM bundle size
- [ ] Test in browser and Node.js

**Deliverables**:
- WASM modules for bundler, nodejs, web
- WASM test suite
- Build scripts

**Success Criteria**:
- ✅ WASM builds successfully for all targets
- ✅ Bundle size < 2MB
- ✅ Tests pass in both browser and Node.js

### Phase 3: TypeScript Wrapper (Week 3)
**Objective**: Create type-safe TypeScript API

**Tasks**:
- [ ] Initialize TypeScript project
- [ ] Create WASM loader with initialization
- [ ] Define TypeScript interfaces and types
- [ ] Build high-level async API
- [ ] Handle errors gracefully
- [ ] Support both ES modules and CommonJS
- [ ] Write TypeScript tests with vitest
- [ ] Generate API documentation

**Deliverables**:
- TypeScript wrapper with .d.ts files
- Comprehensive test suite
- API documentation

**Success Criteria**:
- ✅ Type-safe API with IntelliSense
- ✅ Works in Node.js >= 18
- ✅ ES modules and CommonJS support
- ✅ Test coverage > 80%

### Phase 4: npm Package (Week 4)
**Objective**: Prepare publishable npm package

**Tasks**:
- [ ] Configure package.json with proper exports
- [ ] Bundle WASM files with TypeScript output
- [ ] Write comprehensive README
- [ ] Add usage examples
- [ ] Create CHANGELOG
- [ ] Configure semantic versioning
- [ ] Test installation locally with npm link
- [ ] Verify tree-shaking works
- [ ] Test in sample projects

**Deliverables**:
- Ready-to-publish npm package
- Documentation
- Usage examples

**Success Criteria**:
- ✅ npm pack creates valid tarball
- ✅ Installation works in test project
- ✅ All exports resolve correctly
- ✅ TypeScript types available

### Phase 5: Distroless Container (Week 5)
**Objective**: Build minimal, secure container

**Tasks**:
- [ ] Create multi-stage Dockerfile
- [ ] Use distroless/cc-debian12 base
- [ ] Build native Rust binary (not WASM)
- [ ] Add CLI argument parsing
- [ ] Configure non-root user
- [ ] Test container locally
- [ ] Measure image size
- [ ] Scan for vulnerabilities (trivy)
- [ ] Generate SBOM

**Deliverables**:
- Distroless container image
- Build scripts
- Security scan results

**Success Criteria**:
- ✅ Image size < 50MB
- ✅ No critical vulnerabilities
- ✅ Runs as non-root
- ✅ Fast startup (< 1s)

### Phase 6: GitHub Action (Week 6)
**Objective**: Package as reusable GitHub Action

**Tasks**:
- [ ] Create action.yml with inputs/outputs
- [ ] Build action wrapper script
- [ ] Use container image or npm package
- [ ] Support multiple output formats (JSON, SARIF)
- [ ] Handle errors and exit codes
- [ ] Write action README
- [ ] Test in sample workflows
- [ ] Publish to GitHub Marketplace

**Deliverables**:
- GitHub Action package
- Example workflows
- Documentation

**Success Criteria**:
- ✅ Action runs successfully in workflow
- ✅ Inputs/outputs work correctly
- ✅ SARIF upload to GitHub works
- ✅ Listed on Marketplace

### Phase 7: CI/CD Pipeline (Week 7)
**Objective**: Automate all publishing

**Tasks**:
- [ ] Create build-and-test workflow
- [ ] Create npm publish workflow
- [ ] Create container publish workflow
- [ ] Create GitHub Action publish workflow
- [ ] Configure release workflow
- [ ] Add automated versioning
- [ ] Set up branch protection
- [ ] Configure secrets (NPM_TOKEN, etc.)
- [ ] Test full release cycle

**Deliverables**:
- Complete CI/CD workflows
- Release documentation
- Runbook for releases

**Success Criteria**:
- ✅ Push triggers tests automatically
- ✅ Release publishes to all targets
- ✅ Versioning is automatic
- ✅ Rollback procedure documented

---

## 🧪 Testing Strategy

### Rust Testing
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Benchmarks
cargo bench

# Coverage (with tarpaulin)
cargo tarpaulin --out Html
```

### WASM Testing
```bash
# wasm-bindgen-test (browser)
wasm-pack test --headless --chrome

# wasm-bindgen-test (Node.js)
wasm-pack test --node
```

### TypeScript Testing
```bash
# Unit tests
npm test

# Watch mode
npm test -- --watch

# Coverage
npm test -- --coverage
```

### Container Testing
```bash
# Build test
docker build -t test-image .

# Run test
docker run --rm -v $(pwd):/workspace test-image audit ./src

# Security scan
trivy image test-image

# Size check
docker images test-image
```

### GitHub Action Testing
```bash
# Local testing with act
act -j test-action

# Test in real workflow
git push origin feature/test-action
```

---

## 📊 Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| **WASM bundle size** | < 2MB | wasm-pack build |
| **npm package size** | < 3MB | npm pack --dry-run |
| **Container image** | < 50MB | docker images |
| **Rust compile time** | < 5 min | cargo build --release |
| **WASM build time** | < 3 min | wasm-pack build |
| **Execution speed** | 95% of native | benchmarks |
| **Memory usage** | < 100MB | runtime profiling |
| **Startup time** | < 1s | container start |

---

## 🔒 Security Considerations

### Code Security
- [ ] Use `#![forbid(unsafe_code)]` where possible
- [ ] Run cargo-audit regularly
- [ ] Enable all clippy lints
- [ ] Use dependabot for dependency updates

### Container Security
- [ ] Distroless base (no shell)
- [ ] Non-root user
- [ ] Read-only filesystem where possible
- [ ] Scan with trivy
- [ ] Generate and publish SBOM

### Supply Chain Security
- [ ] Sign releases with GPG
- [ ] Enable npm 2FA
- [ ] Use GitHub's verified commits
- [ ] Pin all action versions
- [ ] Use lock files (Cargo.lock, package-lock.json)

### Secrets Management
- [ ] Never hardcode credentials
- [ ] Use GitHub Secrets for CI/CD
- [ ] Rotate tokens regularly
- [ ] Audit access logs

---

## 📚 Documentation Requirements

### User Documentation
- [ ] **README.md** - Quick start, installation, basic usage
- [ ] **USAGE.md** - Comprehensive usage guide
- [ ] **API.md** - API reference
- [ ] **EXAMPLES.md** - Code examples for each platform
- [ ] **FAQ.md** - Common questions and troubleshooting

### Developer Documentation
- [ ] **CONTRIBUTING.md** - Contribution guidelines
- [ ] **ARCHITECTURE.md** - Technical architecture
- [ ] **DEVELOPMENT.md** - Local setup and development
- [ ] **TESTING.md** - Testing strategy and procedures
- [ ] **RELEASE.md** - Release process

### API Documentation
- [ ] Rust docs (cargo doc)
- [ ] TypeScript docs (TSDoc)
- [ ] GitHub Action docs (action README)

---

## 🚀 Release Strategy

### Versioning
- **Semantic Versioning**: MAJOR.MINOR.PATCH
- **Pre-releases**: 0.1.0-alpha.1, 0.1.0-beta.1, 0.1.0-rc.1
- **Synchronized versions** across npm, container, and action

### Release Process
1. **Prepare Release**
   - Update CHANGELOG.md
   - Bump version in Cargo.toml and package.json
   - Run full test suite
   - Update documentation

2. **Create Release**
   - Tag release: `git tag v0.1.0`
   - Push tag: `git push origin v0.1.0`
   - GitHub Actions automatically:
     - Publishes to npm
     - Builds and pushes container
     - Creates GitHub release

3. **Post-Release**
   - Verify npm package: `npm view @your-org/rust-wasm-app`
   - Verify container: `docker pull ghcr.io/your-org/rust-wasm-app:v0.1.0`
   - Test GitHub Action in sample workflow
   - Announce release

### Release Channels
- **latest** - Stable releases
- **next** - Pre-releases
- **dev** - Development snapshots

---

## 🔧 Development Setup

### Prerequisites
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Node.js >= 18
nvm install 18

# Docker
# Install from docker.com
```

### Local Development
```bash
# Clone repository
git clone https://github.com/your-org/rust-wasm-app.git
cd rust-wasm-app

# Install dependencies
cargo fetch
npm install

# Build everything
npm run build

# Run tests
cargo test
npm test

# Start development
# Terminal 1: Watch Rust
cargo watch -x build

# Terminal 2: Watch TypeScript
npm run dev
```

---

## 📊 Monitoring and Metrics

### Build Metrics
- Build success rate
- Build duration trends
- Artifact size over time
- Test coverage trends

### Usage Metrics
- npm downloads per week
- Container pulls
- GitHub Action usage
- GitHub stars/forks

### Performance Metrics
- Execution time percentiles
- Memory usage patterns
- Error rates
- User-reported issues

---

## 🎯 Success Criteria Summary

### Technical Success
- ✅ Rust code compiles and passes all tests
- ✅ WASM modules work in browser and Node.js
- ✅ TypeScript API is type-safe and intuitive
- ✅ npm package installs and runs correctly
- ✅ Container runs with minimal privileges
- ✅ GitHub Action integrates seamlessly
- ✅ CI/CD pipeline is fully automated

### Quality Success
- ✅ Test coverage > 80% across all layers
- ✅ No critical security vulnerabilities
- ✅ Performance meets targets
- ✅ Documentation is comprehensive
- ✅ Code follows best practices

### Adoption Success
- ✅ Listed on npm with good README
- ✅ Published to GHCR with proper tags
- ✅ Available on GitHub Marketplace
- ✅ Usage examples for all platforms
- ✅ Active community support

---

## 🔮 Future Enhancements

### Phase 2 Features
- [ ] CLI with rich terminal UI (ratatui)
- [ ] VS Code extension
- [ ] Language Server Protocol (LSP)
- [ ] Web playground (WASM in browser)
- [ ] GitLab CI/CD integration
- [ ] Pre-commit hooks
- [ ] IDE plugins (IntelliJ, WebStorm)

### Advanced Features
- [ ] Machine learning for crypto detection
- [ ] Auto-fix suggestions
- [ ] Historical trend analysis
- [ ] Integration with SAST tools
- [ ] Custom rule engine
- [ ] Multi-repo analysis
- [ ] Real-time monitoring

---

## 📞 Support and Resources

### Documentation
- GitHub: https://github.com/your-org/rust-wasm-app
- npm: https://www.npmjs.com/package/@your-org/rust-wasm-app
- Container: https://ghcr.io/your-org/rust-wasm-app
- Action: https://github.com/marketplace/actions/quantum-safe-audit

### Community
- Issues: GitHub Issues
- Discussions: GitHub Discussions
- Chat: Discord or Slack
- Email: support@your-org.com

---

## ✅ Implementation Checklist

### Week 1: Rust Core
- [ ] Cargo project initialized
- [ ] Core audit logic implemented
- [ ] Tests written and passing
- [ ] Benchmarks running

### Week 2: WASM
- [ ] wasm-pack configured
- [ ] WASM builds successfully
- [ ] Browser and Node.js tests pass
- [ ] Bundle size optimized

### Week 3: TypeScript
- [ ] TypeScript wrapper complete
- [ ] Type definitions generated
- [ ] API tests passing
- [ ] Documentation written

### Week 4: npm Package
- [ ] package.json configured
- [ ] README complete
- [ ] Local installation tested
- [ ] Ready to publish

### Week 5: Container
- [ ] Dockerfile created
- [ ] Distroless base configured
- [ ] Security scan passed
- [ ] Image size optimized

### Week 6: GitHub Action
- [ ] action.yml defined
- [ ] Wrapper script working
- [ ] Example workflows tested
- [ ] Marketplace listing ready

### Week 7: CI/CD
- [ ] All workflows created
- [ ] Secrets configured
- [ ] Release process documented
- [ ] Full cycle tested

---

**Plan Status**: 📋 READY FOR IMPLEMENTATION

**Next Steps**:
1. Review and approve this plan
2. Set up GitHub repository
3. Initialize Rust project (Week 1)
4. Follow weekly milestones

**Estimated Timeline**: 7 weeks to production-ready v0.1.0

---

*Architecture Plan v1.0 - January 2025*
*Ready for development kickoff*
