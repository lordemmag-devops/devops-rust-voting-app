# DevOps Rust Voting App

This repository contains a Rust-based voting application, designed with DevOps principles in mind. It includes configurations for Docker, Kubernetes, and Terraform to facilitate easy deployment and management.

## Project Structure

- `docker-compose.yml`: Defines the multi-container Docker application.
- `README.md`: This file.
- `deploy/`: Contains deployment configurations.
  - `deploy/argo-apps/`: ArgoCD application definitions (if used).
  - `deploy/kubernetes/`: Kubernetes manifests.
    - `backend-deployment-blue.yaml`: Kubernetes deployment for blue environment.
    - `backend-deployment-green.yaml`: Kubernetes deployment for green environment.
    - `backend-hpa.yaml`: Horizontal Pod Autoscaler for the backend.
    - `backend-service.yaml`: Kubernetes service for the backend.
    - `kustomization.yaml`: Kustomize base for Kubernetes manifests.
- `infrastructure/`: Infrastructure as Code (IaC) definitions.
  - `infrastructure/terraform/`: Terraform configurations.
    - `main.tf`: Main Terraform configuration file.
    - `outputs.tf`: Terraform output definitions.
    - `services/`: Terraform modules for services.
    - `variables.tf`: Terraform variable definitions.
- `services/`: Contains individual service implementations.
  - `services/backend-rust/`: Rust backend service.
    - `Cargo.toml`: Rust package manifest.
    - `Dockerfile`: Dockerfile for the Rust backend.
    - `src/`: Rust source code.
      - `main.rs`: Main Rust application file.
  - `services/db/`: Database service (e.g., MongoDB).
  - `services/frontend-react/`: React frontend service.

## Getting Started

# ArgoCD Applications

This directory is intended to hold ArgoCD Application definitions.

## Usage

Place your ArgoCD Application YAML files here. These files define how your applications are deployed and managed by ArgoCD.

Example:

```yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: my-rust-app
  namespace: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/lordemmag-devops/devops-rust-voting-app.git
    targetRevision: HEAD
    path: deploy/kubernetes
  destination:
    server: https://kubernetes.default.svc
    namespace: default
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
      
### Prerequisites

- Docker
- Docker Compose
- kubectl
- Terraform

### Local Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/lordemmag-devops/devops-rust-voting-app.git
   cd devops-rust-voting-app
   ```

2. **Start the services using Docker Compose:**
   ```bash
   docker-compose up --build
   ```

   The backend will be accessible at `http://localhost:8080`.

### Kubernetes Deployment

1. **Ensure your `kubectl` is configured to the desired cluster.**

2. **Apply Kubernetes manifests:**
   ```bash
   kubectl apply -k deploy/kubernetes/
   ```

### Terraform Infrastructure

1. **Navigate to the Terraform directory:**
   ```bash
   cd infrastructure/terraform
   ```

2. **Initialize Terraform:**
   ```bash
   terraform init
   ```

3. **Plan and apply the infrastructure:**
   ```bash
   terraform plan
   terraform apply
   ```

## CI/CD

The `.github/workflows/ci-cd.yml` file defines the GitHub Actions workflow for Continuous Integration and Continuous Deployment.

## Contributing

Feel free to contribute to this project by opening issues or submitting pull requests.

## License

This project is licensed under the MIT License.
