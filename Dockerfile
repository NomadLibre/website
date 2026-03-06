FROM rust:trixie

# Install OS dependencies required by GitHub Actions and Rust builds
RUN apt-get update && apt-get install -y \
    curl \
    git \
    libicu-dev \
    sudo \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for the runner
RUN useradd -m actions-runner && \
    usermod -aG sudo actions-runner && \
    echo "%sudo ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Switch to the isolated user
USER actions-runner
WORKDIR /home/actions-runner

# Permanently install the CLI tools into this image
RUN cargo install dioxus-cli cargo-audit

# Ensure the tools are globally available in the path
ENV PATH="/home/actions-runner/.cargo/bin:${PATH}"

# Default to opening a bash shell
CMD ["/bin/bash"]