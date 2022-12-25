FROM rust:1.66-alpine3.17 as build
COPY ./ ./
RUN apk update && apk add build-base protoc protobuf-dev
RUN cargo install --locked --path .

FROM alpine:3.17
COPY --from=build /usr/local/cargo/bin/starlink-exporter /usr/local/bin/
CMD ["/usr/local/bin/starlink-exporter"]
