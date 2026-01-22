FROM --platform=${BUILDPLATFORM} rust:1.89-slim-bookworm AS chef
WORKDIR /app
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo install cargo-chef --locked && \
  rm -rf $CARGO_HOME/registry/

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

RUN apt-get update && apt-get install -y \
  protobuf-compiler \
  perl \
  make \
  git

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin commit-boost-pbs


FROM debian:trixie-20240904-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y \
  openssl \
  ca-certificates \
  libssl3 \
  libssl-dev \
  curl \
  && apt-get clean autoclean \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/commit-boost-pbs /usr/local/bin

RUN groupadd -g 10001 commitboost && \
  useradd -u 10001 -g commitboost -s /sbin/nologin commitboost
USER commitboost

ENTRYPOINT ["/usr/local/bin/commit-boost-pbs"]
