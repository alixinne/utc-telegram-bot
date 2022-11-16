FROM docker.io/library/rust:1.63.0 AS build

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

FROM gcr.io/distroless/cc@sha256:442431d783e435ff0954d2a8214231e3da91ce392c65c1138c0bcdbc77420116
COPY --from=build /src/target/release/utc-telegram-bot /
COPY public /public
ENTRYPOINT ["/utc-telegram-bot"]
CMD ["/utc-telegram-bot", "run", "--bind", "0.0.0.0:3000", "--serve-root", "/public"]
EXPOSE 3000/tcp
