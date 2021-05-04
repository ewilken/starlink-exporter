FROM rust:slim as build
COPY ./ ./
RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo install --path .

FROM debian:10-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=build /usr/local/cargo/bin/starlink-exporter /
CMD ["/starlink-exporter"]
