# Stage 1: Build
FROM rustlang/rust:nightly AS builder
RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    build-essential \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY zion-core/Cargo.toml zion-core/Cargo.lock ./zion-core/
COPY zion-core/src ./zion-core/src
COPY zion-core/proto ./zion-core/proto
COPY zion-core/build.rs ./zion-core/
COPY zion-core/benches ./zion-core/benches
COPY blockrock-core ./blockrock-core
WORKDIR /app/zion-core
RUN cargo +nightly build --release --bin node

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libclang1 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/zion-core/target/release/node /usr/local/bin/
COPY zion-core/static /static
EXPOSE 8000 50051
CMD ["node"]
