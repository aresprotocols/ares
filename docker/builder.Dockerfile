FROM docker.io/paritytech/ci-linux:production as builder
WORKDIR /substrate
COPY . /substrate
# RUN cargo build --locked --release
RUN cargo update && cargo build --release --bin gladios-node --features with-all-runtime

FROM docker.io/library/ubuntu:20.04
COPY --from=builder /substrate/target/release/gladios-node /usr/local/bin
# COPY ./ares/target/release/gladios-node  /usr/local/bin

RUN apt-get update && \
	apt-get install ca-certificates -y && \
	update-ca-certificates && \
	mkdir -p /root/.local/share/gladios-node  && \
	ln -s /root/.local/share/gladios-node /data && \
	/usr/local/bin/gladios-node --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]