FROM docker.io/paritytech/ci-linux:production as builder
WORKDIR /substrate
COPY . /substrate
RUN cargo update
RUN cargo update -p syn:1.0.98 --precise 1.0.96
RUN cargo update -p getrandom:0.2.7 --precise 0.2.3
RUN cargo build --features with-odyssey-runtime --bin gladios-node --profile production --workspace
RUN pwd
RUN ls /substrate
RUN ls /substrate/target
RUN ls /substrate/target/production

FROM docker.io/library/ubuntu:20.04
COPY --from=builder /substrate/target/production/gladios-node /usr/local/bin
#COPY ares_key_01.curl /usr/local/bin
#COPY ares_key_02.curl /usr/local/bin
#COPY ares_key_03.curl /usr/local/bin
#COPY ares_key_04.curl /usr/local/bin
WORKDIR /usr/local/bin

RUN apt-get update && \
apt-get install ca-certificates -y && \
update-ca-certificates && \
mkdir -p /root/.local/share/gladios-node  && \
ln -s /root/.local/share/gladios-node /data && \
/usr/local/bin/gladios-node --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
