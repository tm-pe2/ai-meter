FROM rust:latest AS build

WORKDIR /usr/src/meter

COPY . .
RUN cargo install --path .

FROM debian:buster-slim AS run
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/server /usr/local/bin/server
ENTRYPOINT ["server"]
