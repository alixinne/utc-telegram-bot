FROM docker.io/library/rust:1.57.0 AS build
WORKDIR /src
COPY . /src
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /src/target/release/utc-telegram-bot /
COPY public /public
ENTRYPOINT ["/utc-telegram-bot"]
CMD ["/utc-telegram-bot", "run", "--bind", "0.0.0.0:3000", "--serve-root", "/public"]
EXPOSE 3000/tcp
