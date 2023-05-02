# based on rust official image:
# https://www.docker.com/blog/simplify-your-deployments-using-the-rust-official-image/
#
# postgres library "magic" based on:
# https://stackoverflow.com/questions/74386506/for-docker-that-used-to-work-the-command-bin-sh-returned-a-non-zero-code-100
#
FROM rust:1.68.1 as builder
RUN mkdir -p /log/config/
ADD logging_config.yaml /log/config/logging_config.yaml

WORKDIR /src
COPY . .

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y --no-install-recommends build-essential libpq-dev extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /log/config/logging_config.yaml /log/config/logging_config.yaml
COPY --from=builder /usr/local/cargo/bin/rust-axum-postgres-api-sample /usr/local/bin/rust-axum-postgres-api-sample
CMD ["rust-axum-postgres-api-sample"]
