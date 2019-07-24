FROM rust:latest

RUN rustup update stable

RUN apt-get update
RUN apt-get install -y gnuplot

WORKDIR build
COPY ./Cargo.toml ./Cargo.toml
COPY ./ring ./ring

RUN cargo +stable bench -p crypto_bench_ring
