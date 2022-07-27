FROM docker.io/paritytech/ci-linux:production as builder
WORKDIR /substrate
COPY . /substrate
RUN pwd
RUN ls ./
RUN cargo update
RUN cargo update -p syn:1.0.98 --precise 1.0.96
RUN pwd
RUN ls ./
RUN cargo build --features with-pioneer-runtime,with-pioneer-fast-runtime --bin ares-node --profile production --workspace
RUN pwd
RUN ls ./
RUN ls ./target
RUN ls ./target/production
RUN ls /
RUN ls /substrate
RUN ls /substrate/target/production


FROM docker.io/library/ubuntu:20.04
COPY --from=builder /substrate/target/production/gladios-node /usr/local/bin
# COPY ./ares/target/release/gladios-node  /usr/local/bin

RUN apt-get update && \
	apt-get install ca-certificates -y && \
	update-ca-certificates && \
	mkdir -p /root/.local/share/ares-node  && \
	ln -s /root/.local/share/ares-node /data && \
	/usr/local/bin/ares-node --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]