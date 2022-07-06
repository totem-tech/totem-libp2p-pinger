# Usage

# This builds the Totem LibP2P ping tester.

# docker build \
# -t totemlive/totem-libp2p-ping:local \
# -f ping_builder.Dockerfile .

# This is the build stage for Totem Parachain. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /rust-libp2p
COPY . /rust-libp2p

# rust compiler command 
RUN cargo build --release --example ping

# This is the 2nd stage: a very small image where we copy the Totem Parachain Collator binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Totem LibP2P Ping Test" \
	totem.live.image.type="builder" \
	totem.live.image.authors="chris.dcosta@totemaccounting.com" \
	totem.live.image.vendor="Totem Accounting" \
	totem.live.image.description="Totem is a p2p accounting engine for the decentralised economy 🚀" \
	totem.live.image.source="https://github.com/totem-tech/totem-libp2p-ping/ping_builder.Dockerfile" \
	totem.live.image.documentation="https://github.com/totem-tech/totem-libp2p-ping"

COPY --from=builder /rust-libp2p/target/release/examples/ping /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /rust-libp2p totemadmin && \
/usr/local/bin/ping 30333

USER totemadmin

CMD /usr/local/bin/ping