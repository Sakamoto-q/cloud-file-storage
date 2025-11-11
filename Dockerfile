FROM debian:bookworm-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y curl unzip ca-certificates build-essential && rm -rf /var/lib/apt/lists/*

COPY . .

RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

WORKDIR /app/frontend
RUN bun install
RUN bun run build

RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /root/

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app .
COPY --from=builder /app/target/release/cloud-file-storage ./

CMD ["./cloud-file-storage"]