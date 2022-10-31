FROM docker.io/library/rust:1.63.0 AS build
WORKDIR /src
COPY . /src
RUN cargo build --release

FROM gcr.io/distroless/cc@sha256:3827818d6d0c62a2b03fbced0a0ccd4ffdf98095f4a536fb878d5fc2d8bfb049
COPY --from=build /src/target/release/utc-telegram-bot /
COPY public /public
ENTRYPOINT ["/utc-telegram-bot"]
CMD ["/utc-telegram-bot", "run", "--bind", "0.0.0.0:3000", "--serve-root", "/public"]
EXPOSE 3000/tcp
