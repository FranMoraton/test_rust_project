FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin test_rust_project
WORKDIR /test_rust_project

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/test_rust_project*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /test_rust_project/target/release/test_rust_project .

# set the startup command to run your binary
CMD ["./test_rust_project"]