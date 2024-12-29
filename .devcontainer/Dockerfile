FROM rust:1.77-slim

ARG USER_NAME=typst-jp
ARG USER_UID=1000
ARG USER_GID=$USER_UID
ARG BUN_VERSION=1.1.37

RUN groupadd --gid $USER_GID $USER_NAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USER_NAME \
    && chsh -s /bin/bash $USER_NAME \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        git \
        openssh-client \
        curl \
        unzip \
        python3 \
        python3-pip \
    && python3 -m pip install --break-system-packages jinja2 PyYAML \
    && su $USER_NAME -c 'curl -fsSL https://bun.sh/install | bash -s "bun-v$BUN_VERSION"' \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
ENV PATH="/root/.cargo/bin:${PATH}"
USER $USER_NAME
