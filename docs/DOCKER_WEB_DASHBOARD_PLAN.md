# Docker Web Dashboard & API Server - Feature Plan

## Executive Summary

This document outlines a non-destructive, value-added feature for PQC Scanner: a containerized web dashboard and REST API server that provides interactive scanning capabilities, persistent result storage, and enhanced user experience through a modern web interface.

## Feature Overview

### Core Capabilities

1. **Interactive Web Dashboard**
   - Upload files/directories for scanning
   - Real-time scan progress indicators
   - Interactive compliance report viewer
   - Historical scan result tracking
   - Export reports (JSON, HTML, PDF)
   - Comparison view for multiple scans

2. **REST API Server**
   - RESTful endpoints for programmatic access
   - Authentication/API key support
   - Webhook notifications
   - Batch scanning capabilities
   - OpenAPI/Swagger documentation

3. **Docker Containerization**
   - Multi-stage build for minimal image size
   - Health check endpoints
   - Volume mounts for persistent data
   - Environment-based configuration
   - GitHub Container Registry (ghcr.io) publishing

## Value Proposition

### Why This Feature Adds Value

1. **Accessibility**: Web UI lowers barrier to entry for non-technical users
2. **Integration**: REST API enables CI/CD pipeline integration
3. **Persistence**: Historical tracking enables trend analysis
4. **Deployment**: Docker simplifies deployment in enterprise environments
5. **Non-Destructive**: Preserves all existing CLI functionality
6. **Scalability**: Can run as microservice in cloud environments

### Target Users

- **Security Teams**: Dashboard for centralized vulnerability tracking
- **DevOps Engineers**: API integration for automated scanning
- **Compliance Officers**: Historical reports for audit trails
- **Developers**: Quick web-based scans without CLI setup
- **Enterprise**: Containerized deployment in private clouds

## Technical Architecture

### Technology Stack

```
┌─────────────────────────────────────────────┐
│           Frontend (Web Dashboard)          │
│  • React + TypeScript                       │
│  • Chart.js for visualizations              │
│  • File upload with drag-and-drop           │
│  • Real-time WebSocket updates              │
└─────────────────────────────────────────────┘
                      ↓ HTTP/WebSocket
┌─────────────────────────────────────────────┐
│         Backend (REST API Server)           │
│  • Rust + Actix-web framework               │
│  • JWT authentication                        │
│  • OpenAPI/Swagger docs                      │
│  • WebSocket for real-time updates          │
└─────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────┐
│         PQC Scanner Core Library            │
│  • Existing Rust/WASM audit engine          │
│  • No modifications required                │
│  • Called via existing public API           │
└─────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────┐
│         Data Persistence Layer              │
│  • SQLite for scan results                  │
│  • File storage for uploaded code           │
│  • JSON export capabilities                 │
└─────────────────────────────────────────────┘
```

### REST API Endpoints

```
POST   /api/v1/scan              # Upload and scan files
GET    /api/v1/scan/:id          # Get scan results
GET    /api/v1/scans             # List all scans
DELETE /api/v1/scan/:id          # Delete scan results
GET    /api/v1/scan/:id/report   # Export report (JSON/HTML/PDF)
POST   /api/v1/scan/compare      # Compare multiple scans
GET    /api/v1/health            # Health check endpoint
GET    /api/v1/metrics           # Prometheus metrics
GET    /api/docs                 # OpenAPI documentation
```

### Docker Architecture

```dockerfile
# Multi-stage build
FROM rust:1.82-alpine AS builder
  # Build Rust backend

FROM node:22-alpine AS frontend-builder
  # Build React frontend

FROM alpine:3.20 AS runtime
  # Minimal runtime image
  # < 100MB final size
```

## Implementation Plan

### Phase 1: Backend API Server (Week 1-2)

#### Dependencies
```toml
# Add to Cargo.toml
actix-web = "4.9"
actix-cors = "0.7"
actix-files = "0.6"
tokio = { version = "1.40", features = ["full"] }
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio"] }
jsonwebtoken = "9.3"
utoipa = "5.2"         # OpenAPI docs
utoipa-swagger-ui = "8.0"
```

#### File Structure
```
src/
├── bin/
│   ├── pqc-scanner.rs        # Existing CLI (unchanged)
│   └── pqc-server.rs         # NEW: Web server binary
├── server/
│   ├── mod.rs                # Server module entry
│   ├── routes.rs             # API endpoint handlers
│   ├── models.rs             # Data models
│   ├── db.rs                 # Database operations
│   ├── auth.rs               # Authentication
│   └── websocket.rs          # WebSocket handler
```

#### Core Features
- File upload endpoint with multipart/form-data
- Integration with existing `analyze()` function
- SQLite storage for scan results
- JWT-based authentication
- OpenAPI documentation generation

### Phase 2: Frontend Dashboard (Week 2-3)

#### Technology
- React 18 + TypeScript
- Vite for build tooling
- TailwindCSS for styling
- Chart.js for visualizations
- React Router for navigation

#### Pages
1. **Dashboard** - Overview of recent scans
2. **Upload** - Drag-and-drop file upload
3. **Results** - Interactive scan results viewer
4. **History** - List of all scans with filtering
5. **Compare** - Side-by-side comparison
6. **Settings** - Configuration options

#### File Structure
```
web/
├── src/
│   ├── App.tsx
│   ├── pages/
│   │   ├── Dashboard.tsx
│   │   ├── Upload.tsx
│   │   ├── Results.tsx
│   │   └── History.tsx
│   ├── components/
│   │   ├── FileUpload.tsx
│   │   ├── ScanProgress.tsx
│   │   ├── ComplianceChart.tsx
│   │   └── VulnerabilityList.tsx
│   └── api/
│       └── client.ts          # API client
├── package.json
└── vite.config.ts
```

### Phase 3: Docker Containerization (Week 3-4)

#### Multi-Stage Dockerfile
```dockerfile
# Stage 1: Build Rust backend
FROM rust:1.82-alpine AS rust-builder
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /build
COPY Cargo.* ./
COPY src/ ./src/
COPY data/ ./data/
RUN cargo build --release --bin pqc-server

# Stage 2: Build frontend
FROM node:22-alpine AS frontend-builder
WORKDIR /build
COPY web/package*.json ./
RUN npm ci
COPY web/ ./
RUN npm run build

# Stage 3: Runtime image
FROM alpine:3.20
RUN apk add --no-cache ca-certificates sqlite
WORKDIR /app

# Copy binaries and assets
COPY --from=rust-builder /build/target/release/pqc-server .
COPY --from=frontend-builder /build/dist ./static
COPY data/ ./data/

# Create data directory for SQLite
RUN mkdir -p /app/data

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s \
  CMD wget -q --spider http://localhost:8080/api/v1/health || exit 1

CMD ["./pqc-server"]
```

#### Docker Compose for Development
```yaml
version: '3.8'
services:
  pqc-scanner-web:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - ./data:/app/data
      - ./uploads:/app/uploads
    environment:
      - DATABASE_URL=sqlite:///app/data/pqc-scanner.db
      - RUST_LOG=info
      - JWT_SECRET=${JWT_SECRET}
    healthcheck:
      test: ["CMD", "wget", "-q", "--spider", "http://localhost:8080/api/v1/health"]
      interval: 30s
      timeout: 3s
      retries: 3
```

### Phase 4: CI/CD Integration (Week 4)

#### GitHub Actions Workflow
```yaml
name: Build and Publish Docker Image

on:
  push:
    branches: [main]
    tags: ['v*']
  pull_request:
    branches: [main]

jobs:
  build-and-push:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - uses: actions/checkout@v4

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Log in to GitHub Container Registry
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
            type=ref,event=branch
            type=ref,event=pr
            type=semver,pattern={{version}}
            type=semver,pattern={{major}}.{{minor}}
            type=sha
            type=raw,value=latest,enable={{is_default_branch}}

      - name: Build and push
        uses: docker/build-push-action@v6
        with:
          context: .
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
          platforms: linux/amd64,linux/arm64
```

## Configuration

### Environment Variables
```bash
# Server configuration
PORT=8080
HOST=0.0.0.0

# Database
DATABASE_URL=sqlite:///app/data/pqc-scanner.db

# Authentication
JWT_SECRET=<random-secret-key>
JWT_EXPIRY=86400  # 24 hours

# Storage
UPLOAD_DIR=/app/uploads
MAX_UPLOAD_SIZE=104857600  # 100MB

# Logging
RUST_LOG=info
```

### Configuration File (config.toml)
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[upload]
max_size = 104857600  # 100MB
allowed_extensions = ["rs", "js", "ts", "py", "java", "go", "cpp", "cs"]

[database]
url = "sqlite:///app/data/pqc-scanner.db"
max_connections = 5

[auth]
jwt_secret = "${JWT_SECRET}"
jwt_expiry_seconds = 86400

[features]
enable_auth = false  # Disable for demo/testing
enable_metrics = true
enable_swagger = true
```

## Usage Examples

### Docker Run
```bash
# Pull from GitHub Container Registry
docker pull ghcr.io/arcqubit/pqc-scanner:latest

# Run with default settings
docker run -p 8080:8080 ghcr.io/arcqubit/pqc-scanner:latest

# Run with persistent data
docker run -p 8080:8080 \
  -v $(pwd)/data:/app/data \
  -v $(pwd)/uploads:/app/uploads \
  -e JWT_SECRET=my-secret-key \
  ghcr.io/arcqubit/pqc-scanner:latest

# Access dashboard
open http://localhost:8080
```

### Docker Compose
```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### API Usage
```bash
# Upload and scan a file
curl -X POST http://localhost:8080/api/v1/scan \
  -F "file=@vulnerable_code.py" \
  -F "language=python"

# Get scan results
curl http://localhost:8080/api/v1/scan/{scan_id}

# Export as JSON
curl http://localhost:8080/api/v1/scan/{scan_id}/report?format=json > report.json

# List all scans
curl http://localhost:8080/api/v1/scans
```

### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pqc-scanner
spec:
  replicas: 3
  selector:
    matchLabels:
      app: pqc-scanner
  template:
    metadata:
      labels:
        app: pqc-scanner
    spec:
      containers:
      - name: pqc-scanner
        image: ghcr.io/arcqubit/pqc-scanner:latest
        ports:
        - containerPort: 8080
        env:
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: pqc-scanner-secrets
              key: jwt-secret
        volumeMounts:
        - name: data
          mountPath: /app/data
        livenessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: pqc-scanner-data
```

## Security Considerations

1. **Authentication**: JWT-based auth for API access
2. **File Validation**: Strict file type and size validation
3. **Sandboxing**: Uploaded files processed in isolated context
4. **Rate Limiting**: Prevent abuse of API endpoints
5. **CORS**: Configurable CORS policies
6. **Input Sanitization**: Prevent injection attacks
7. **HTTPS**: TLS/SSL support via reverse proxy (nginx/traefik)

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Container Image Size | < 100MB | Multi-stage build optimization |
| Cold Start Time | < 3s | Alpine-based runtime |
| API Response Time | < 100ms | Excluding scan time |
| Scan Throughput | 1000 LOC/ms | Existing performance maintained |
| Concurrent Requests | 100+ | Actix-web async handling |
| Memory Usage | < 256MB | Per container |

## Documentation Updates

### New Documentation Files
1. `docs/WEB_DASHBOARD.md` - User guide for web interface
2. `docs/API_REFERENCE.md` - REST API documentation
3. `docs/DOCKER_DEPLOYMENT.md` - Docker deployment guide
4. `docs/KUBERNETES.md` - Kubernetes deployment guide

### README Updates
- Add "Web Dashboard" section
- Add "Docker Deployment" quick start
- Update installation options with Docker
- Add screenshots of web interface

## Testing Strategy

### Backend Tests
```rust
#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn test_upload_endpoint() { }

    #[actix_web::test]
    async fn test_scan_results() { }

    #[actix_web::test]
    async fn test_authentication() { }
}
```

### Frontend Tests
```typescript
// Using Vitest + React Testing Library
describe('FileUpload Component', () => {
  it('should handle file upload', () => { });
  it('should show progress indicator', () => { });
});
```

### Integration Tests
```bash
# Docker build test
./scripts/test-docker-build.sh

# E2E tests with Playwright
npm run test:e2e
```

## Migration Path

### For Existing Users
1. **CLI remains unchanged** - No breaking changes
2. **Optional feature** - Users can continue using CLI only
3. **Backward compatible** - All existing APIs preserved
4. **Gradual adoption** - Can migrate workflows incrementally

### Deployment Options
- **CLI only**: `cargo install pqc-scanner`
- **Docker CLI**: `docker run ghcr.io/arcqubit/pqc-scanner scan ./code`
- **Web dashboard**: `docker run -p 8080:8080 ghcr.io/arcqubit/pqc-scanner`
- **API server**: Deploy container with API endpoints enabled

## Success Metrics

### Adoption Metrics
- Docker image pull count
- Web dashboard active users
- API endpoint usage
- GitHub stars/forks increase

### Performance Metrics
- Container startup time
- API response latency
- Scan throughput maintained
- Resource utilization

### User Satisfaction
- GitHub issue resolution time
- Feature request implementation
- Community feedback (surveys)

## Future Enhancements

### Phase 5+ (Post-Launch)
1. **Multi-tenancy**: Support for multiple organizations
2. **LDAP/SAML**: Enterprise authentication
3. **Report Templates**: Customizable report formats
4. **Scheduled Scans**: Cron-based automated scanning
5. **Slack/Email Notifications**: Integration with notification services
6. **Vulnerability Database**: CVE mapping and tracking
7. **Remediation Suggestions**: AI-powered fix recommendations
8. **CI/CD Plugins**: Jenkins, GitLab CI, CircleCI integrations
9. **Compliance Packs**: Pre-configured compliance profiles
10. **Export to SIEM**: Integration with security tools (Splunk, ELK)

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Increased complexity | Medium | Maintain clear separation of concerns |
| Docker image bloat | Low | Multi-stage builds, Alpine base |
| Security vulnerabilities | High | Regular dependency updates, security scanning |
| Performance regression | Medium | Comprehensive benchmarking |
| Maintenance burden | Medium | Automated testing, clear documentation |

## Conclusion

This feature adds significant value to PQC Scanner by:
- Making the tool more accessible via web interface
- Enabling enterprise deployment via Docker
- Providing programmatic access via REST API
- Maintaining backward compatibility with existing CLI

The non-destructive nature ensures existing users are unaffected while new capabilities attract broader adoption in enterprise environments.

## References

- Actix Web: https://actix.rs/
- Docker Multi-stage Builds: https://docs.docker.com/build/building/multi-stage/
- GitHub Container Registry: https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry
- OpenAPI/Swagger: https://swagger.io/specification/
- NIST 800-53 SC-13: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf
