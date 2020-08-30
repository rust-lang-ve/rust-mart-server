FROM rust:latest as build

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/server

COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/server /usr/local/bin/server

CMD ["server"]
