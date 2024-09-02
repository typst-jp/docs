# Rust v1.77 as a base image
FROM rust:1.77-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    # install python3, jinja2 pyyaml
    python3 \
    python3-pip \
    && python3 -m pip install --break-system-packages jinja2 PyYAML \
    # install nodejs and serve
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g serve@14.2.3 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \

WORKDIR /workspace
ENV PATH="/root/.cargo/bin:${PATH}"
