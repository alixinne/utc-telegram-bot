FROM docker.io/library/rust:1.57.0 AS build
WORKDIR /src
COPY . /src
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /src/target/release/utc-telegram-bot /
ENTRYPOINT ["/utc-telegram-bot"]
