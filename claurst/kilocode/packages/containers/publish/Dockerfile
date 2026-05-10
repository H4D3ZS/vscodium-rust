# kilocode_change start
ARG REGISTRY=ghcr.io/kilo-org
# kilocode_change end
FROM ${REGISTRY}/build/bun-node:24.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
  docker.io \
  pacman-package-manager \
  && rm -rf /var/lib/apt/lists/*
