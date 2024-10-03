FROM rust:1.81.0 as builder

WORKDIR /usr/src/push-byceps-stats-to-discord

# Create project to build and cache dependencies.
RUN cargo init --bin
COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo build --release && \
    rm ./src/main.rs && \
    rm ./target/release/deps/push_byceps_stats_to_discord*

# Add and compile actual source code.
COPY ./src ./src
RUN cargo build --release

FROM rust:1.81.0-slim-bookworm
COPY --from=builder /usr/src/push-byceps-stats-to-discord/target/release/push-byceps-stats-to-discord .
CMD ["./push-byceps-stats-to-discord", "--config", "config.toml"]
