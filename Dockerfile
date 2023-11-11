FROM rust:1.70.0-slim-bullseye as builder
WORKDIR subitobot
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/subitobot/target \
cargo build --release && \
    mv /subitobot/target/release/subitobot /subitobot

FROM gcr.io/distroless/cc-debian11
WORKDIR /subitobot
VOLUME /subitobot/data
COPY --from=builder /subitobot/subitobot ./subitobot/subitobot
CMD ["./subitobot/subitobot"]