# Setup rust build environment
FROM rust:1.46.0 AS build-context

RUN rustup component add rustfmt clippy

WORKDIR /usr/src/
COPY event-models ./event-models
WORKDIR /usr/src/workspace-service

COPY workspace-service/Cargo.toml .
COPY workspace-service/Cargo.lock .

ENV SQLX_OFFLINE=true

# Layer hack: Build an empty program to compile dependencies and place on their own layer.
# This cuts down build time

# it was borrowed from here:
# https://github.com/deislabs/krustlet/blob/master/Dockerfile#L7
RUN mkdir -p ./src/ && echo 'fn main() {}' >./src/main.rs && echo '' >./src/lib.rs

RUN cargo fetch

RUN cargo build --release --locked && rm -rf ./target/release/.fingerprint/workspace_service-*

# Setup debian release environment
FROM debian:buster-slim AS release-context

RUN apt-get update && apt-get install -y \
    tini \
    ;

RUN apt install libssl1.1 -y

RUN useradd svc

# Build real binaries now, as late as possible
FROM build-context AS build

COPY workspace-service/src ./src
COPY workspace-service/sql ./sql
COPY workspace-service/sqlx-data.json .
COPY workspace-service/migrations ./migrations

RUN cargo clippy --release -- -D warnings && cargo build --release

# This target is skipped in the normal `docker build` flow because it requires the database
# Use `docker build --target test` to run it
FROM build AS test
ARG TEST_DATABASE_URL=postgres://postgres:postgres@172.17.0.1:5432
ENV TEST_DATABASE_URL=$TEST_DATABASE_URL

RUN cargo test --release

# Create the release
FROM release-context AS release

COPY --from=build /usr/src/workspace-service/target/release/workspace_service /

RUN chown -R svc /workspace_service

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/workspace_service"]
