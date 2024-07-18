FROM docker.io/library/rust:1.74.0 AS build

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

FROM gcr.io/distroless/cc-debian12@sha256:3b75fdd33932d16e53a461277becf57c4f815c6cee5f6bc8f52457c095e004c8
COPY --from=build /src/target/release/utc-telegram-bot /
COPY public /public
ENTRYPOINT ["/utc-telegram-bot"]
CMD ["/utc-telegram-bot", "run", "--bind", "0.0.0.0:3000", "--serve-root", "/public"]
EXPOSE 3000/tcp
