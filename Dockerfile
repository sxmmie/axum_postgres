# Stage 1: Planner - compute depeendencies recipe

FROM rust:1.91-bookworm-slim AS builder
WORKDIR /app