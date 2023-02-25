FROM rust:1.67 as builder

RUN update-ca-certificates

WORKDIR /subitobot
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /subitobot
COPY --from=builder /subitobot/target/release/subitobot ./
CMD ["/subitobot/subitobot"]