FROM ares-builder:latest as builder
WORKDIR /substrate
COPY . /substrate
# RUN cargo build --locked --release
RUN cargo update && cargo build --release --bin gladios-node

FROM docker.io/library/ubuntu:20.04
COPY --from=builder /substrate/target/release/ares-node /usr/local/bin

RUN apt-get update && \
	apt-get install ca-certificates -y && \
	update-ca-certificates && \
	mkdir -p /root/.local/share/gladios-node  && \
	ln -s /root/.local/share/gladios-node /data && \
	/usr/local/bin/gladios-node --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]