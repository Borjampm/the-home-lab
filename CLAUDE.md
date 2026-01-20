# CLAUDE.md - AI Assistant Guide for the-home-lab

**Last Updated:** 2026-01-20
**Project Stage:** Inception/Planning Phase
**Current Branch:** `claude/add-claude-documentation-ccMUP`

---

## 📋 Project Overview

**Repository:** the-home-lab
**Owner:** Borjampm (Borja Márquez de la Plata)
**Purpose:** A personal home lab infrastructure project for connecting computers and other utilities

### Current State
This is a **brand new repository** in the initial planning stage. As of this documentation:
- ✅ Repository initialized with git
- ✅ Basic README.md created
- ✅ Documentation structure established
- ⏳ No source code implemented yet
- ⏳ Technology stack not yet selected
- ⏳ Infrastructure design in progress

---

## 📁 Repository Structure

```
the-home-lab/
├── README.md                    # Project entry point and overview
├── CLAUDE.md                    # This file - AI assistant guide
├── docs/                        # Documentation directory
│   ├── architecture/            # System design and architecture docs
│   ├── development/             # Development guides and workflows
│   ├── infrastructure/          # Infrastructure specifications
│   └── conventions/             # Coding standards and conventions
└── .git/                        # Git repository metadata
```

### Documentation Organization

All documentation is organized in the `docs/` folder with topic-specific sub-folders:

- **`docs/architecture/`** - System design, component diagrams, architecture decisions
- **`docs/development/`** - Development workflows, setup guides, contributing guidelines
- **`docs/infrastructure/`** - Hardware specs, network topology, deployment configs
- **`docs/conventions/`** - Coding standards, naming conventions, best practices

---

## 🔍 Research Phase

### Entry Point
**Start with:** `README.md` - This provides the high-level project vision and goals.

### Research Checklist
When analyzing or contributing to this repository, AI assistants should:

- [ ] Read `README.md` to understand project goals
- [ ] Review all documentation in `docs/` folders
- [ ] Check for any configuration files (when they exist)
- [ ] Understand the target infrastructure and requirements
- [ ] Identify technology stack decisions (when made)
- [ ] Review any existing architecture documents
- [ ] Understand deployment and operational requirements

---

## 🛠️ Development Workflows

### Git Branch Strategy
**Main Development Branch:** Not yet established
**Current Feature Branch:** `claude/add-claude-documentation-ccMUP`

### Branch Naming Convention
- Feature branches: `claude/<description>-<sessionId>`
- All Claude AI development must happen on designated feature branches
- Never push directly to main/master without permission

### Commit Practices
- Use clear, descriptive commit messages
- Follow conventional commit format when applicable
- Sign commits with GPG (pre-configured in this repo)
- Group related changes in single commits

### Git Operations
**For git push:**
```bash
git push -u origin <branch-name>
```
- Branch must start with 'claude/' and end with matching session ID
- Retry up to 4 times with exponential backoff on network errors (2s, 4s, 8s, 16s)

**For git fetch/pull:**
```bash
git fetch origin <branch-name>
git pull origin <branch-name>
```
- Retry up to 4 times with exponential backoff on network errors

---

## 🏗️ Technology Stack

### Current Status: **NOT YET DETERMINED**

This section will be updated as technology decisions are made. Expected categories:

- **Languages:** TBD (Python, Bash, Go, etc.)
- **Infrastructure as Code:** TBD (Terraform, Ansible, Docker Compose, etc.)
- **Container Orchestration:** TBD (Docker, Kubernetes, Nomad, etc.)
- **Configuration Management:** TBD
- **CI/CD:** TBD (GitHub Actions, Jenkins, etc.)
- **Monitoring:** TBD (Prometheus, Grafana, etc.)

---

## 📐 Code Conventions

### General Principles
- Keep solutions simple and focused
- Avoid over-engineering
- Only make changes that are directly requested or clearly necessary
- Don't add features beyond what was asked
- Delete unused code completely (no backwards-compatibility hacks)

### Security Considerations
- Never introduce security vulnerabilities (XSS, SQL injection, command injection, etc.)
- Validate input at system boundaries (user input, external APIs)
- Don't skip git hooks unless explicitly requested
- Never commit secrets or credentials

### Documentation Standards
- Update documentation when making changes
- Keep CLAUDE.md synchronized with repository state
- Document architectural decisions
- Include setup and deployment instructions
- Use clear, concise language

---

## 🔑 Key Files and Their Purpose

| File | Purpose | Status |
|------|---------|--------|
| `README.md` | Project overview and entry point | ✅ Created |
| `CLAUDE.md` | AI assistant guide (this file) | ✅ Created |
| `docs/architecture/` | System design documentation | 📁 Directory created |
| `docs/development/` | Development guides | 📁 Directory created |
| `docs/infrastructure/` | Infrastructure specifications | 📁 Directory created |
| `docs/conventions/` | Standards and conventions | 📁 Directory created |

---

## 🎯 AI Assistant Guidelines

### When Working on This Repository

1. **Always start with research**
   - Read README.md first
   - Review relevant documentation in docs/
   - Understand the current project phase

2. **Use TodoWrite for task management**
   - Plan work before implementation
   - Track progress with todos
   - Mark tasks complete immediately after finishing

3. **Follow git workflow**
   - Develop on designated feature branches only
   - Use descriptive commit messages
   - Push to correct branch with -u flag
   - Never force push to main/master

4. **Maintain documentation**
   - Update CLAUDE.md when repository structure changes
   - Keep docs/ folder organized by topic
   - Document decisions and rationale

5. **Ask questions when unclear**
   - Use AskUserQuestion tool for clarification
   - Don't make assumptions about requirements
   - Validate architectural decisions

### What NOT to Do

- ❌ Don't create files unnecessarily
- ❌ Don't over-engineer solutions
- ❌ Don't add features not requested
- ❌ Don't push to wrong branches
- ❌ Don't commit secrets or credentials
- ❌ Don't skip reading existing code/docs
- ❌ Don't use bash commands for communication (output text directly)

---

## 📊 Project Status

### Phase: **Planning & Documentation**

**Completed:**
- [x] Repository initialization
- [x] Basic README created
- [x] Documentation structure established
- [x] CLAUDE.md guide created

**In Progress:**
- [ ] Architecture design
- [ ] Technology stack selection
- [ ] Infrastructure planning

**Not Started:**
- [ ] Code implementation
- [ ] CI/CD setup
- [ ] Testing framework
- [ ] Deployment automation

---

## 📚 Additional Resources

### Documentation Index
- Architecture docs: `docs/architecture/`
- Development guides: `docs/development/`
- Infrastructure specs: `docs/infrastructure/`
- Conventions: `docs/conventions/`

### Git Configuration
- **Remote:** `http://local_proxy@127.0.0.1:30445/git/Borjampm/the-home-lab`
- **GPG Signing:** Enabled (SSH format)
- **User:** Claude (noreply@anthropic.com)

---

## 🔄 Changelog

### 2026-01-20
- Initial CLAUDE.md creation
- Documentation structure established
- Research phase completed
- Repository exploration documented

---

## 💡 Notes for Future Development

As this project evolves, update this document with:
- Technology stack choices and rationale
- Architecture decisions and patterns
- Deployment procedures
- Operational runbooks
- Testing strategies
- Security considerations
- Performance requirements

---

**Remember:** This is a living document. Keep it updated as the project grows and evolves.
