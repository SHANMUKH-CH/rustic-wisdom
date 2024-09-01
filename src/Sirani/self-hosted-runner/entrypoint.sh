#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Check if required environment variables are set
if [ -z "$REPO_URL" ] || [ -z "$RUNNER_TOKEN" ]; then
  echo "REPO_URL and RUNNER_TOKEN must be set"
  exit 1
fi

# Configure the runner
./config.sh --url "$REPO_URL" --token "$RUNNER_TOKEN" --unattended --replace

# Run the runner
./run.sh