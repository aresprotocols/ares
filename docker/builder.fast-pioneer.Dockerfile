FROM docker.io/paritytech/ci-linux:production as builder
WORKDIR /substrate
COPY . /substrate
RUN cargo update
RUN cargo update -p syn:1.0.98 --precise 1.0.96
RUN cargo build --features with-pioneer-runtime,with-pioneer-fast-runtime --bin gladios-node --profile production --workspace

FROM docker.io/library/ubuntu:20.04
RUN ls /
RUN ls ./
RUN ls ./target
RUN ls ./target/production
COPY --from=builder ./target/production/gladios-node /usr/local/bin
# COPY ./ares/target/release/gladios-node  /usr/local/bin

RUN apt-get update && \
	apt-get install ca-certificates -y && \
	update-ca-certificates && \
	mkdir -p /root/.local/share/gladios-node  && \
	ln -s /root/.local/share/gladios-node /data && \
	/usr/local/bin/gladios-node --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]