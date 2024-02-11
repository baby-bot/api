# Stage 1: Building the binary
# Use the official Rust image as a builder stage
FROM rust:latest as builder

# # Create a new empty shell project
# RUN USER=root cargo new --bin myapp
# WORKDIR /myapp

# Copy your manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now that dependencies are cached, copy your source code
COPY ./src ./src

# Build your application
# RUN rm ./target/release/deps/myapp*
# RUN cargo check
RUN cargo build --release

# Stage 2: Creating the runtime image
FROM debian:buster-slim

# Copy the build artifact from the builder stage
COPY --from=builder /myapp/target/release/myapp .

# Set the binary as the entrypoint of the container
ENTRYPOINT ["./myapp"]
