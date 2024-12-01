# Rust v1.77 as a base image
FROM rust:1.77-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        git \
        openssh-client \
        python3 \
        python3-pip \
    && python3 -m pip install --break-system-packages jinja2 PyYAML \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
ENV PATH="/root/.cargo/bin:${PATH}"
