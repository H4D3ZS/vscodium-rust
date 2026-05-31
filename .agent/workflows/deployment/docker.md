---
description: Containerize application with Docker
---

# Docker

I will help you containerize your application with Docker.

## Guardrails
- Detect existing Dockerfile before creating
- Follow Docker best practices for image size
- Don't include secrets in images
- Use multi-stage builds when appropriate

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What type of application? (Node, Python, etc.)
- Single container or multi-container?
- Any specific base image requirements?
- Need docker-compose?

### 2. Analyze Application
Determine container needs:
- Runtime requirements
- Dependencies to install
- Ports to expose
- Files to copy

### 3. Create Dockerfile
Follow best practices:
- Use specific base image tags
- Order layers for caching
- Use multi-stage for smaller images
- Copy only necessary files

### 4. Create docker-compose (if needed)
For multi-container apps:
- Define services
- Set up networking
- Configure volumes
- Add health checks

### 5. Build and Test
- Build the image
- Run container locally
- Test functionality
- Check image size

### 6. Verify
- Container starts correctly
- Application works
- Logs are accessible

## Principles
- Keep images small
- Don't run as root
- Use .dockerignore
- Tag images properly
