FROM rust:1.77-slim

ARG USERNAME=typst-jp
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && chsh -s /bin/bash $USERNAME \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        git \
        openssh-client \
        python3 \
        python3-pip \
    && python3 -m pip install --break-system-packages jinja2 PyYAML \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
ENV PATH="/root/.cargo/bin:${PATH}"
USER $USERNAME
