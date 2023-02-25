FROM rust:1.67 as builder

RUN update-ca-certificates

WORKDIR /subitobot
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/subitobot/target \
cargo build --release && \
    mv /subitobot/target/release/subitobot /subitobot

FROM gcr.io/distroless/cc
WORKDIR /subitobot
VOLUME /subitobot/data
COPY --from=builder /subitobot/subitobot ./
CMD ["/subitobot/subitobot"]