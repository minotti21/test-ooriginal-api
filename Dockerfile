FROM rust:1.82 as builder

RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /usr/src/ooriginal-qrcode-api

COPY Cargo.toml Cargo.lock ./

ENV DATABASE_URL="postgresql://postgres.pdxpjrgcttjuxrvabbkd:QyzvWcYR2i3QxiDC@aws-0-sa-east-1.pooler.supabase.com:5432/postgres"

RUN mkdir src
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 libpq5

WORKDIR /usr/src/ooriginal-qrcode-api

COPY --from=builder /usr/src/ooriginal-qrcode-api/target/release/ooriginal-qrcode-api .

EXPOSE 8080

CMD ["./ooriginal-qrcode-api"]