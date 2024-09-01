SHELL := /bin/bash

NETWORK_NAME=temperance-elk-network
REPO=rustic-wisdom
SIRANI_PATH=/mnt/d/${REPO}/src/Sirani
DOCKER_COMPOSE_PATH=${SIRANI_PATH}/docker-compose.yml
SELF_HOSTED_RUNNER_PATH=${SIRANI_PATH}/self-hosted-runner
ES_HOST=localhost
ES_PORT=9200
ES_USER=elastic
ES_PASS=changeme
KIBANA_HOST=localhost
KIBANA_PORT=5601
OWNER=SHANMUKH-CH

.PHONY: all token build create-network run-self-hosted-runner docker-up

all: token build create-network run-self-hosted-runner docker-up

token:
	${SELF_HOSTED_RUNNER_PATH}/get-token.sh ${OWNER} ${REPO}
	@echo ">>> runnertoken.env file contents after token target:"
	@cat runnertoken.env

build:
	@source runnertoken.env && docker build --build-arg REPO_URL=$$REPO_URL --build-arg RUNNER_TOKEN=$$RUNNER_TOKEN -t self-hosted-runner:${REPO} -f ${SELF_HOSTED_RUNNER_PATH}/Dockerfile ${SELF_HOSTED_RUNNER_PATH}

create-network:
	docker network inspect $(NETWORK_NAME) >/dev/null 2>&1 || docker network create $(NETWORK_NAME)

docker-up:
	docker-compose -f $(DOCKER_COMPOSE_PATH) up -d

.ONESHELL:
