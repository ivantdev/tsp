# Etapa 1: Construcción del binario
FROM rust:slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY . .

RUN cargo build --release

# Etapa 2: Imagen para ejecución
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y curl ca-certificates libpq5 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/tsp .

COPY static ./static
COPY templates ./templates
RUN curl https://objects.ivant.dev/public/projects/tsp/nodes.txt --output nodes.txt
RUN curl https://objects.ivant.dev/public/projects/tsp/edges.txt --output edges.txt

EXPOSE 8000

CMD ["/usr/local/bin/tsp"]