FROM lukemathwalker/cargo-chef:latest-rust-1.70.0-alpine3.18 as chef
WORKDIR subitobot

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /subitobot/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /subitobot/target/release/subitobot /subitobot/subitobot
COPY . /subitobot/data
CMD ["./subitobot/subitobot"]