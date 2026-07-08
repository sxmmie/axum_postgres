################################
# Stage 1: Planner - compute dependency recipe
################################

FROM rust:1.91-bookworm-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

################################
# Stage 2: Builder - cache deps, then build app
################################