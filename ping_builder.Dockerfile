# docker build \
# -t totemlive/totem-libp2p-ping:local \
# -f ping_builder.Dockerfile .

FROM debian:stretch-slim as builder

LABEL description="Multistage Docker image for Totem LibP2P Ping Test" \
	totem.live.image.type="builder" \
	totem.live.image.authors="chris.dcosta@totemaccounting.com" \

# show backtraces
ENV RUST_BACKTRACE 1

WORKDIR /totem-libp2p-pinger
COPY . /totem-libp2p-pinger

# install tools and dependencies
RUN apt-get update && \
		DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
		DEBIAN_FRONTEND=noninteractive apt-get install -y \
			cmake \
            pkg-config \
            libssl1.1 \
            git \
            clang \
			ca-certificates \
			curl

# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup toolchain install nightly && \
	rustup default nightly && \
# apt cleanup
		apt-get autoremove -y && \
		apt-get clean && \
		find /var/lib/apt/lists/ -type f -not -name lock -delete;

# rust compiler command to build binary 
RUN cargo build --release --example ping

# This is the 2nd stage: a very small image where we copy the Totem LibP2P Pinger binary."
# FROM docker.io/library/ubuntu:20.04
FROM debian:stretch-slim

LABEL description="Multistage Docker image for Totem LibP2P Ping Test" \
	totem.live.image.type="container" \
	totem.live.image.authors="chris.dcosta@totemaccounting.com" \
	totem.live.image.vendor="Totem Accounting" \
	totem.live.image.description="Totem is a p2p accounting engine for the decentralised economy 🚀" \
	totem.live.image.source="https://github.com/totem-tech/totem-libp2p-ping/ping_builder.Dockerfile" \
	totem.live.image.documentation="https://github.com/totem-tech/totem-libp2p-ping"

COPY --from=builder /totem-libp2p-pinger/target/release/examples/ping /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /totem-libp2p-pinger totemadmin

# run a check
# /usr/local/bin/ping 30333

USER totemadmin

CMD /usr/local/bin/ping