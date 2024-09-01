FROM ubuntu:latest

# Accept build arguments
ARG REPO_URL
ARG RUNNER_TOKEN
ARG RUNNER_VERSION=2.319.1

# Set environment variables
ENV REPO_URL=${REPO_URL}
ENV RUNNER_TOKEN=${RUNNER_TOKEN}
ENV RUNNER_VERSION=${RUNNER_VERSION}

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    jq \
    git \
    sudo \
    libicu-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a user
RUN useradd -m runner
RUN echo "runner ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Switch to the runner user
USER runner
WORKDIR /home/runner

# Download and install GitHub Actions runner
RUN curl -o actions-runner-linux-x64.tar.gz -L https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-x64-${RUNNER_VERSION}.tar.gz \
    && tar xzf ./actions-runner-linux-x64.tar.gz \
    && rm ./actions-runner-linux-x64.tar.gz

# Run the install dependencies script with sudo
RUN sudo ./bin/installdependencies.sh

# Copy entrypoint script
COPY entrypoint.sh /home/runner/entrypoint.sh
RUN sudo chmod +x /home/runner/entrypoint.sh

ENTRYPOINT ["/home/runner/entrypoint.sh"]