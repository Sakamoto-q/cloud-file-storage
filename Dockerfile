FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

FROM rust:slim AS runtime

WORKDIR /root/

COPY --from=builder /app .
COPY --from=builder /app/target/release/cloud-file-storage ./

CMD ["./cloud-file-storage"]