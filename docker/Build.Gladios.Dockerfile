FROM docker.io/paritytech/ci-linux:production as builder
WORKDIR /substrate
COPY . /substrate
# RUN cargo update
# RUN cargo update -p syn:1.0.98 --precise 1.0.96
# RUN rustup install 1.62.1
# RUN rustup default 1.62.1
# RUN rustup update nightly
RUN cargo build --features with-gladios-runtime --bin ares-node --profile production --workspace
RUN pwd
RUN ls /substrate
RUN ls /substrate/target
RUN ls /substrate/target/production

FROM docker.io/library/ubuntu:20.04
COPY --from=builder /substrate/target/production/ares-node /usr/local/bin
#COPY ares_key_01.curl /usr/local/bin
#COPY ares_key_02.curl /usr/local/bin
#COPY ares_key_03.curl /usr/local/bin
#COPY ares_key_04.curl /usr/local/bin
WORKDIR /usr/local/bin

RUN apt-get update && \
apt-get install ca-certificates -y && \
update-ca-certificates && \
mkdir -p /root/.local/share/ares-node  && \
ln -s /root/.local/share/ares-node /data && \
/usr/local/bin/ares-node --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]