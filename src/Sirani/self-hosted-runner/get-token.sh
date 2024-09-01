#!/usr/bin/bash

set -xe

declare token
declare owner
declare repo
declare REPO_URL
declare RUNNER_TOKEN

owner="$1"
repo="$2"

if [ -z "$owner" ] || [ -z "$repo" ]; then
    echo "Usage: $0 <owner> <repo>"
    exit 1
fi

if gh auth status > /dev/null 2>&1; then
    token=$(gh api --method POST -H "Accept: application/vnd.github.v3+json" repos/${owner}/${repo}/actions/runners/registration-token | jq -r .token)
else
    echo ">>> Not logged in to github.com as $owner"
    exit 1
fi

if [ -z "$token" ]; then
    echo ">>> Failed to get registration token"
    exit 1
else
    REPO_URL="https://github.com/${owner}/${repo}"
    RUNNER_TOKEN="${token}"
    echo "REPO_URL=${REPO_URL}" > runnertoken.env
    echo "RUNNER_TOKEN=${RUNNER_TOKEN}" >> runnertoken.env
    echo ">>> runnertoken.env file contents:"
    cat runnertoken.env
fi