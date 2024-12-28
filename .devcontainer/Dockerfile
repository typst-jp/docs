FROM rust:1.77-slim

ARG USERNAME=typst-jp
ARG USER_UID=1000
ARG USER_GID=$USER_UID
ARG BUN_VERSION=1.1.37

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && chsh -s /bin/bash $USERNAME \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        git \
        openssh-client \
        curl \
        unzip \
        python3 \
        python3-pip \
    && python3 -m pip install --break-system-packages jinja2 PyYAML \
    && su $USERNAME -c 'curl -fsSL https://bun.sh/install | bash -s "bun-v$BUN_VERSION"' \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
ENV PATH="/root/.cargo/bin:${PATH}"
USER $USERNAME
