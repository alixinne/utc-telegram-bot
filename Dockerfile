FROM docker.io/library/rust:1.72.1 AS build

WORKDIR /src

# Build dependencies
COPY Cargo.lock Cargo.toml /src/
RUN mkdir -p src && \
    echo 'fn main() {}' >src/main.rs && \
    cargo build --release

# Deploy code
COPY alphabets.txt /src/
COPY migrations /src/migrations
COPY src /src/src
RUN touch -am src/main.rs && cargo build --release

FROM gcr.io/distroless/cc@sha256:fb402c45f3ef485ccd56ca2af2a58615fc47c4978bb3004e8663a83456791f48
COPY --from=build /src/target/release/utc-telegram-bot /
COPY public /public
ENTRYPOINT ["/utc-telegram-bot"]
CMD ["/utc-telegram-bot", "run", "--bind", "0.0.0.0:3000", "--serve-root", "/public"]
EXPOSE 3000/tcp
