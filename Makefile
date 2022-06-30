#.PHONY: push-docker build-docker build-docker-from-local

TOP_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

## get the latest commit hash in the short form
#COMMIT := $(shell git rev-parse --short HEAD)
#REPOSITORY := aresprotocollab/ares_gladios

# all of above variables, could be overrode by .env
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

#push-docker:
#	docker tag ${REPOSITORY}:${COMMIT} ${REPOSITORY}:latest
#	docker push ${REPOSITORY}:${COMMIT}
#	docker push ${REPOSITORY}:latest
#
#build-docker:
#	docker build --build-arg NGINX_CONF=${NGINX_CONF} --build-arg ENV_CONFIG=${BUILDER_CONFIGURATION} -t ${REPOSITORY}:${COMMIT} -f Dockerfile .

.PHONY: run
run:
	cargo run -- --dev -lruntime=debug --instant-sealing

.PHONY: run-gladios
run-gladios:
	cargo run -- --chain=gladios

run-odyssey:
	cargo run -- --chain=odyssey

.PHONY: build-release
build-release:
	cargo build --locked --features with-all-runtime --profile production --workspace

.PHONY: build-gladios
build-gladios:
	cargo build --locked --features with-gladios-runtime --bin gladios-node --profile production --workspace

.PHONY: build-odyssey
build-odyssey:
	cargo build --locked --features with-odyssey-runtime --bin odyssey-node --profile production --workspace

.PHONY: build-pioneer
build-pioneer:
	cargo build --locked --features with-pioneer-runtime --bin gladios-node --profile production --workspace

.PHONY: build-gladios-fast
build-gladios-fast:
	cargo build --locked --features with-gladios-runtime,with-gladios-fast-runtime --bin gladios-node --profile production --workspace

.PHONY: build-odyssey-fast
build-odyssey-fast:
	cargo build --locked --features with-odyssey-runtime,with-odyssey-fast-runtime --bin gladios-node --profile production --workspace

.PHONY: build-pioneer-fast
build-pioneer-fast:
	cargo build --locked --features with-pioneer-runtime,with-pioneer-fast-runtime --bin gladios-node --profile production --workspace