# ArgoCD GitOps Deployment for pqc-scanner

This directory contains ArgoCD configuration for GitOps-based continuous deployment of the pqc-scanner application to Kubernetes clusters.

## Overview

ArgoCD provides declarative, GitOps continuous delivery for Kubernetes with:

- **Declarative Setup**: All configuration in Git
- **Automated Sync**: Auto-deploy on Git commits
- **Health Monitoring**: Track application health
- **Rollback Capability**: Easy rollback to previous versions
- **Multi-Environment Support**: Dev, Staging, Production
- **Image Updates**: Automated image version updates
- **Self-Healing**: Automatically fix drift from desired state

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Git Repository                       │
│  ┌─────────────────────────────────────────────────┐   │
│  │ argocd/                                          │   │
│  │  ├── application.yaml (ArgoCD App definition)   │   │
│  │  └── manifests/                                 │   │
│  │      ├── deployment.yaml                        │   │
│  │      ├── service.yaml                           │   │
│  │      ├── configmap.yaml                         │   │
│  │      ├── serviceaccount.yaml                    │   │
│  │      ├── hpa.yaml                               │   │
│  │      └── kustomization.yaml                     │   │
│  └─────────────────────────────────────────────────┘   │
└────────────────┬────────────────────────────────────────┘
                 │ Git Sync
                 ▼
┌─────────────────────────────────────────────────────────┐
│                    ArgoCD Server                        │
│  ┌────────────────────────────────────────────────┐    │
│  │  • Monitors Git repository                     │    │
│  │  • Detects configuration changes               │    │
│  │  • Validates manifests                         │    │
│  │  • Syncs to Kubernetes                         │    │
│  └────────────────────────────────────────────────┘    │
└────────────────┬────────────────────────────────────────┘
                 │ Apply
                 ▼
┌─────────────────────────────────────────────────────────┐
│              Kubernetes Cluster                         │
│  ┌────────────────────────────────────────────────┐    │
│  │ Namespace: pqc-scanner                         │    │
│  │  ├── Deployment (pqc-scanner)                  │    │
│  │  ├── Service                                   │    │
│  │  ├── ConfigMap                                 │    │
│  │  ├── ServiceAccount                            │    │
│  │  └── HorizontalPodAutoscaler                   │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## Prerequisites

### 1. Kubernetes Cluster

You need a running Kubernetes cluster. Options:

- **Local**: Minikube, kind, k3s
- **Cloud**: EKS (AWS), GKE (Google Cloud), AKS (Azure)
- **On-Premise**: Self-hosted Kubernetes

Verify cluster access:

```bash
kubectl cluster-info
kubectl get nodes
```

### 2. ArgoCD Installation

Install ArgoCD in your cluster:

```bash
# Create namespace
kubectl create namespace argocd

# Install ArgoCD
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Wait for ArgoCD to be ready
kubectl wait --for=condition=available --timeout=600s \
  deployment/argocd-server -n argocd
```

### 3. Access ArgoCD UI

```bash
# Port-forward ArgoCD server
kubectl port-forward svc/argocd-server -n argocd 8080:443

# Get initial admin password
kubectl -n argocd get secret argocd-initial-admin-secret \
  -o jsonpath="{.data.password}" | base64 -d && echo

# Login via UI
# URL: https://localhost:8080
# Username: admin
# Password: (from command above)
```

### 4. ArgoCD CLI (Optional)

```bash
# Install ArgoCD CLI
brew install argocd  # macOS
# or
curl -sSL -o /usr/local/bin/argocd https://github.com/argoproj/argo-cd/releases/latest/download/argocd-linux-amd64
chmod +x /usr/local/bin/argocd

# Login via CLI
argocd login localhost:8080 --insecure
```

## Setup Instructions

### 1. Create Application Namespace

```bash
# Create namespace for pqc-scanner
kubectl create namespace pqc-scanner

# Optional: Create namespaces for other environments
kubectl create namespace pqc-scanner-dev
kubectl create namespace pqc-scanner-staging
```

### 2. Configure Image Pull Secrets (if needed)

For private container registries:

```bash
# Create Docker registry secret
kubectl create secret docker-registry ghcr-credentials \
  --docker-server=ghcr.io \
  --docker-username=YOUR_GITHUB_USERNAME \
  --docker-password=YOUR_GITHUB_TOKEN \
  --docker-email=YOUR_EMAIL \
  -n pqc-scanner

# Repeat for other namespaces
kubectl create secret docker-registry ghcr-credentials \
  --docker-server=ghcr.io \
  --docker-username=YOUR_GITHUB_USERNAME \
  --docker-password=YOUR_GITHUB_TOKEN \
  --docker-email=YOUR_EMAIL \
  -n pqc-scanner-dev
```

Update `deployment.yaml` to use the secret:

```yaml
spec:
  template:
    spec:
      imagePullSecrets:
        - name: ghcr-credentials
```

### 3. Deploy ArgoCD Application

#### Option A: Using kubectl

```bash
# Deploy production application
kubectl apply -f argocd/application.yaml

# Verify application created
kubectl get applications -n argocd
```

#### Option B: Using ArgoCD CLI

```bash
# Create application from file
argocd app create -f argocd/application.yaml

# Or create interactively
argocd app create pqc-scanner \
  --repo https://github.com/arcqubit/pqc-scanner.git \
  --path argocd/manifests \
  --dest-server https://kubernetes.default.svc \
  --dest-namespace pqc-scanner
```

#### Option C: Using ArgoCD UI

1. Login to ArgoCD UI
2. Click **+ NEW APP**
3. Fill in details:
   - **Application Name**: pqc-scanner
   - **Project**: default
   - **Sync Policy**: Automatic
   - **Repository URL**: https://github.com/arcqubit/pqc-scanner.git
   - **Path**: argocd/manifests
   - **Cluster URL**: https://kubernetes.default.svc
   - **Namespace**: pqc-scanner
4. Click **CREATE**

### 4. Verify Deployment

```bash
# Check ArgoCD application status
argocd app get pqc-scanner

# Check Kubernetes resources
kubectl get all -n pqc-scanner

# Check deployment status
kubectl rollout status deployment/pqc-scanner -n pqc-scanner

# Check pods
kubectl get pods -n pqc-scanner

# View pod logs
kubectl logs -f deployment/pqc-scanner -n pqc-scanner
```

## Multi-Environment Setup

The configuration includes three environments:

### 1. Development (dev)

- **Namespace**: `pqc-scanner-dev`
- **Branch**: `develop`
- **Auto-sync**: Enabled
- **Image**: `ghcr.io/arcqubit/pqc-scanner:develop`

```bash
# Deploy dev environment
kubectl apply -f argocd/application.yaml
# (The file includes pqc-scanner-dev application)
```

### 2. Staging

- **Namespace**: `pqc-scanner-staging`
- **Branch**: `main`
- **Auto-sync**: Disabled (manual approval)
- **Image**: `ghcr.io/arcqubit/pqc-scanner:latest`

```bash
# Sync staging manually
argocd app sync pqc-scanner-staging
```

### 3. Production

- **Namespace**: `pqc-scanner`
- **Branch**: `main`
- **Auto-sync**: Enabled
- **Image**: `ghcr.io/arcqubit/pqc-scanner:latest`

## GitOps Workflow

### Deployment Process

1. **Commit Changes**: Update manifests in Git
2. **ArgoCD Detection**: ArgoCD detects changes (every 3 minutes)
3. **Validation**: ArgoCD validates manifests
4. **Sync**: ArgoCD applies changes to cluster
5. **Health Check**: ArgoCD monitors deployment health
6. **Notification**: Send alerts on sync status

### Making Changes

#### Update Container Image

```bash
# Edit kustomization.yaml
cd argocd/manifests
kustomize edit set image ghcr.io/arcqubit/pqc-scanner:2025.11.1

# Commit and push
git add kustomization.yaml
git commit -m "chore: update image to 2025.11.1"
git push origin main

# ArgoCD will auto-sync (if enabled)
```

#### Update Configuration

```bash
# Edit configmap.yaml
vim argocd/manifests/configmap.yaml

# Commit and push
git add argocd/manifests/configmap.yaml
git commit -m "config: update scan timeout"
git push origin main
```

#### Scale Deployment

```bash
# Edit deployment.yaml
vim argocd/manifests/deployment.yaml
# Change replicas: 2 to replicas: 4

# Commit and push
git add argocd/manifests/deployment.yaml
git commit -m "scale: increase replicas to 4"
git push origin main
```

### Manual Sync

Force immediate sync:

```bash
# Via CLI
argocd app sync pqc-scanner

# Via UI
# Click on application → SYNC → SYNCHRONIZE
```

### Rollback

Revert to previous version:

```bash
# List history
argocd app history pqc-scanner

# Rollback to specific revision
argocd app rollback pqc-scanner 5

# Or revert Git commit and sync
git revert HEAD
git push origin main
argocd app sync pqc-scanner
```

## Monitoring and Observability

### Application Health

```bash
# Check overall health
argocd app get pqc-scanner

# Watch sync status
argocd app wait pqc-scanner --health
```

### Resource Status

```bash
# List all resources
kubectl get all -n pqc-scanner

# Check specific resources
kubectl describe deployment pqc-scanner -n pqc-scanner
kubectl describe hpa pqc-scanner -n pqc-scanner

# View events
kubectl get events -n pqc-scanner --sort-by='.lastTimestamp'
```

### Logs

```bash
# Application logs
kubectl logs -f deployment/pqc-scanner -n pqc-scanner

# ArgoCD application controller logs
kubectl logs -f deployment/argocd-application-controller -n argocd

# ArgoCD server logs
kubectl logs -f deployment/argocd-server -n argocd
```

### Metrics

If Prometheus is installed:

```bash
# Port-forward Prometheus
kubectl port-forward -n monitoring svc/prometheus 9090:9090

# Query metrics
# - argocd_app_sync_total
# - argocd_app_health_status
```

## Advanced Configuration

### 1. Automated Image Updates

Install ArgoCD Image Updater:

```bash
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj-labs/argocd-image-updater/stable/manifests/install.yaml
```

Annotations are already in `application.yaml`:

```yaml
annotations:
  argocd-image-updater.argoproj.io/image-list: pqc-scanner=ghcr.io/arcqubit/pqc-scanner
  argocd-image-updater.argoproj.io/pqc-scanner.update-strategy: semver
```

### 2. Notifications

Install ArgoCD Notifications:

```bash
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj-labs/argocd-notifications/stable/manifests/install.yaml
```

Configure Slack notifications:

```bash
# Create secret with Slack token
kubectl create secret generic argocd-notifications-secret \
  --from-literal=slack-token=YOUR_SLACK_TOKEN \
  -n argocd

# Configure notification service
kubectl apply -f - <<EOF
apiVersion: v1
kind: ConfigMap
metadata:
  name: argocd-notifications-cm
  namespace: argocd
data:
  service.slack: |
    token: \$slack-token
  trigger.on-deployed: |
    - when: app.status.operationState.phase in ['Succeeded']
      send: [app-deployed]
  template.app-deployed: |
    message: Application {{.app.metadata.name}} deployed to {{.app.spec.destination.namespace}}
    slack:
      attachments: |
        [{
          "title": "{{.app.metadata.name}}",
          "color": "good",
          "fields": [{
            "title": "Sync Status",
            "value": "{{.app.status.sync.status}}",
            "short": true
          }]
        }]
EOF
```

Subscribe application:

```yaml
# In application.yaml
annotations:
  notifications.argoproj.io/subscribe.on-deployed.slack: security-team
```

### 3. Progressive Delivery with Argo Rollouts

Install Argo Rollouts:

```bash
kubectl create namespace argo-rollouts
kubectl apply -n argo-rollouts -f https://github.com/argoproj/argo-rollouts/releases/latest/download/install.yaml
```

Convert Deployment to Rollout:

```yaml
apiVersion: argoproj.io/v1alpha1
kind: Rollout
metadata:
  name: pqc-scanner
spec:
  replicas: 3
  strategy:
    canary:
      steps:
        - setWeight: 20
        - pause: {duration: 5m}
        - setWeight: 50
        - pause: {duration: 5m}
        - setWeight: 80
        - pause: {duration: 5m}
  # ... rest of deployment spec
```

### 4. Multi-Cluster Deployment

Add external cluster to ArgoCD:

```bash
# Get kubeconfig for external cluster
export KUBECONFIG=~/.kube/config-external

# Add cluster to ArgoCD
argocd cluster add external-cluster-context

# Update application.yaml
spec:
  destination:
    server: https://external-cluster-api-server
    namespace: pqc-scanner
```

## Security Best Practices

### 1. RBAC Configuration

Limit ArgoCD permissions:

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: argocd-pqc-scanner
rules:
  - apiGroups: ["*"]
    resources: ["*"]
    verbs: ["get", "list", "watch"]
  - apiGroups: ["apps"]
    resources: ["deployments"]
    verbs: ["*"]
```

### 2. Resource Quotas

```bash
kubectl create quota pqc-scanner-quota \
  --hard=requests.cpu=2,requests.memory=2Gi,limits.cpu=4,limits.memory=4Gi \
  -n pqc-scanner
```

### 3. Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: pqc-scanner-netpol
spec:
  podSelector:
    matchLabels:
      app: pqc-scanner
  policyTypes:
    - Ingress
    - Egress
  ingress:
    - from:
        - podSelector:
            matchLabels:
              app: ingress-nginx
  egress:
    - to:
        - podSelector: {}
```

### 4. Pod Security Standards

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: pqc-scanner
  labels:
    pod-security.kubernetes.io/enforce: restricted
    pod-security.kubernetes.io/audit: restricted
    pod-security.kubernetes.io/warn: restricted
```

## Troubleshooting

### Application Not Syncing

```bash
# Check ArgoCD application status
argocd app get pqc-scanner

# Check sync errors
argocd app get pqc-scanner --show-operation

# Force refresh
argocd app get pqc-scanner --refresh
```

### Pods Not Starting

```bash
# Check pod status
kubectl get pods -n pqc-scanner

# Describe pod
kubectl describe pod POD_NAME -n pqc-scanner

# Check events
kubectl get events -n pqc-scanner --sort-by='.lastTimestamp'

# Check logs
kubectl logs POD_NAME -n pqc-scanner
```

### Image Pull Errors

```bash
# Verify image exists
docker pull ghcr.io/arcqubit/pqc-scanner:latest

# Check image pull secret
kubectl get secret ghcr-credentials -n pqc-scanner -o yaml

# Recreate secret if needed
kubectl delete secret ghcr-credentials -n pqc-scanner
kubectl create secret docker-registry ghcr-credentials \
  --docker-server=ghcr.io \
  --docker-username=USERNAME \
  --docker-password=TOKEN \
  -n pqc-scanner
```

### Out of Sync

```bash
# Check diff
argocd app diff pqc-scanner

# Hard refresh
argocd app get pqc-scanner --hard-refresh

# Delete and recreate
argocd app delete pqc-scanner
kubectl apply -f argocd/application.yaml
```

## Performance Optimization

### 1. Reduce Sync Interval

```yaml
# In application.yaml
spec:
  source:
    repoURL: https://github.com/arcqubit/pqc-scanner.git
    # Add sync timeout
    syncOptions:
      - Timeout=5m
```

### 2. Resource Requests/Limits

Tune based on actual usage:

```yaml
resources:
  requests:
    memory: "64Mi"
    cpu: "50m"
  limits:
    memory: "128Mi"
    cpu: "200m"
```

### 3. HPA Tuning

Adjust scaling parameters:

```yaml
spec:
  minReplicas: 1
  maxReplicas: 5
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          averageUtilization: 80
```

## Cleanup

### Remove Application

```bash
# Delete ArgoCD application (will delete all resources)
argocd app delete pqc-scanner

# Or via kubectl
kubectl delete application pqc-scanner -n argocd
```

### Uninstall ArgoCD

```bash
# Delete ArgoCD
kubectl delete -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Delete namespace
kubectl delete namespace argocd
```

## References

- [ArgoCD Documentation](https://argo-cd.readthedocs.io/)
- [Kustomize Documentation](https://kustomize.io/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [ArgoCD Image Updater](https://argocd-image-updater.readthedocs.io/)
- [Argo Rollouts](https://argoproj.github.io/argo-rollouts/)
- [GitOps Principles](https://opengitops.dev/)
- [CalVer Specification](../docs/CALVER.md)
- [Project README](../README.md)
