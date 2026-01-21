# Home Lab Technology Stacks

**Research Date:** 2026-01-20
**Sources:** GitHub repositories, technical blogs, implementation guides

---

## Overview

Common technology combinations and infrastructure-as-code approaches used in modern home labs.

---

## Popular Stack Combinations

### The 2026 Standard Stack

**Core Components:**
1. Mini PCs (hardware layer)
2. Proxmox VE (hypervisor)
3. Docker Swarm or Kubernetes (container orchestration)
4. Git (version control)
5. Reverse proxy (Nginx/Caddy/Traefik)

**Advanced Features:**
- Software-defined storage with Ceph
- Proxmox Backup Server (PBS)
- Terraform for infrastructure provisioning
- Ansible for configuration management

---

## Infrastructure as Code (IaC)

### Terraform + Proxmox
- Terraform provisions VMs and infrastructure
- Ability to practice spinning up/down K8s nodes of different sizes
- Templates allow single-command VM creation
- No long manual VM/LXC creation wizards

### Ansible for Configuration
- Automates Kubernetes deployment
- Modifies system settings post-provisioning
- Network configuration automation
- Desktop customization tools
- Playbooks for consistency across nodes

---

## Common Architecture Patterns

### Pattern 1: Proxmox → Terraform → K3s/Kubernetes

**Layer 1: Hypervisor**
- Proxmox VE as base hypervisor
- Handles VM lifecycle

**Layer 2: Provisioning**
- Terraform creates VMs programmatically
- Infrastructure defined as code
- Version controlled configurations

**Layer 3: Orchestration**
- K3s (Rancher's lightweight Kubernetes)
- Full Kubernetes for learning/production-like setups
- Container workloads

**Layer 4: Supporting Services**
- Calico for networking
- OpenEBS for volume provisioning
- MetalLB for network load balancing

### Pattern 2: Proxmox → Docker Compose

**Simpler Alternative:**
- Proxmox for virtualization
- Docker Compose for container management
- Good for beginners
- Less complexity than Kubernetes

---

## Kubernetes on Home Labs

### Why Kubernetes?

**Learning Value:**
- Hands-on with production-grade orchestration
- Practice for professional skills
- Understanding cloud-native patterns

**Practical Benefits:**
- Service discovery
- Load balancing
- Self-healing capabilities
- Rolling updates

### K3s vs Full Kubernetes

**K3s (Rancher):**
- Lightweight distribution
- Lower resource requirements
- Perfect for home labs
- Production-ready despite size

**Full Kubernetes:**
- More resource-intensive
- Closer to enterprise setups
- Better for learning full stack

---

## Essential Supporting Tools

### Networking
- **Calico** - Kubernetes networking and security
- **MetalLB** - Load balancer for bare metal K8s
- **Traefik/Nginx** - Ingress controllers

### Storage
- **OpenEBS** - Container-native storage
- **Ceph** - Distributed storage for Proxmox
- **Longhorn** - Lightweight distributed storage

### Monitoring & Observability
- **Prometheus** - Metrics collection
- **Grafana** - Visualization
- **Loki** - Log aggregation

### Backup & Recovery
- **Proxmox Backup Server** - VM backups
- **Velero** - Kubernetes backup
- **Restic** - Generic backup tool

---

## Container Management Options

### Komodo (2025/2026 trend)
- Newest option
- Free and modern
- Built-in management features
- Growing community adoption

### Portainer
- Mature and stable
- Great web UI
- Good for beginners
- Agent-based architecture

### Code-Based Management
- Docker Compose files
- Kubernetes manifests
- GitOps workflows (ArgoCD, Flux)
- Preferred by experienced users

---

## Real-World Implementation Examples

### Example 1: Full Production-Like Setup
```
Hardware: Mini PC or server
↓
Proxmox VE 9.1
↓
Terraform (infrastructure provisioning)
↓
Kubernetes cluster (3+ nodes)
├── Calico (networking)
├── OpenEBS (storage)
├── MetalLB (load balancing)
└── Applications (deployed via Helm/ArgoCD)
```

### Example 2: Beginner-Friendly Setup
```
Hardware: Used office PC
↓
Proxmox VE
↓
Docker + Portainer
└── Docker Compose stacks
```

### Example 3: Hybrid Approach
```
Hardware: Mini PCs (2-3 units)
↓
Proxmox VE cluster
├── K3s cluster (containers)
├── Traditional VMs (non-containerized apps)
└── Storage VMs (NAS, backup)
```

---

## Technology Selection Criteria

### Consider Your Goals

**Learning Focus:**
- Full Kubernetes for depth
- Terraform + Ansible for IaC skills
- Production-like technologies

**Practical Self-Hosting:**
- Docker Compose for simplicity
- Proxmox for flexibility
- Focus on reliability

**Experimentation:**
- Mix of technologies
- Easy rebuild capability
- Snapshot-friendly setups

### Resource Constraints

**Limited Hardware (16-32GB RAM):**
- Docker Compose
- K3s if wanting Kubernetes
- LXC containers in Proxmox

**Moderate Hardware (64GB+ RAM):**
- Full Kubernetes
- Multiple VMs
- Ceph storage

**Multi-Node Setup:**
- Proxmox cluster
- Kubernetes cluster
- Distributed storage (Ceph/Longhorn)

---

## Sources

- [The Ultimate Kubernetes Homelab Guide - datastrophic](https://datastrophic.io/kubernetes-homelab-with-proxmox-kubeadm-calico-openebs-and-metallb/)
- [Ultimate Home Lab Starter Stack for 2026 - Virtualization Howto](https://www.virtualizationhowto.com/2025/12/ultimate-home-lab-starter-stack-for-2026-key-recommendations/)
- [Home Lab | homelab-monorepo](https://blog.clayshekleton.com/homelab-monorepo/)
- [The ultimate kubernetes homelab setup - DEV Community](https://dev.to/gjrdiesel/the-ultimate-kubernetes-homelab-setup-558f)
- [GitHub - Vinetos/infrastructure](https://github.com/Vinetos/infrastructure)
- [Deploying a Kubernetes cluster on Proxmox using Terraform and Ansible](https://olav.ninja/deploying-kubernetes-cluster-on-proxmox-part-1)
- [Setting Up The Home Lab: Kubernetes Using Ansible - DEV Community](https://dev.to/popefelix/setting-up-the-home-lab-setting-up-kubernetes-using-ansible-3ji1)
- [GitHub - ehlesp/smallab-k8s-pve-guide](https://github.com/ehlesp/smallab-k8s-pve-guide)
