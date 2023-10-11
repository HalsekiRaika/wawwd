FROM rust:1.72.0 AS build-stage

RUN mkdir /wawwd
WORKDIR /wawwd

COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

COPY migrations ./migrations
COPY application ./application
COPY driver ./driver
COPY kernel ./kernel
COPY server ./server

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build-stage /wawwd/target/release/server /

CMD ["/server"]