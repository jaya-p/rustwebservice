# Build: docker build -t djayap/rustwebservice -f Multistage.Dockerfile .
#         && docker images
# Run: docker run --rm -p 8080:8080 djayap/rustwebservice // docker run -p <host-port>:<container-port> <image-name>
# Test: curl http://localhost:8080/

# STAGE 1 is to build the binary
# Use rust-based image for container; rustc version 1.43.1
# use alpine for musl. musl for supporting static binary.
FROM rust:1.43.1-alpine AS builder

# Set working directory in container; make directory if not exists
RUN mkdir -p /usr/src/rustwebservice
WORKDIR /usr/src/rustwebservice

# Copy all source code file from local computer to container
COPY src src
COPY Cargo.toml .
COPY LICENSE .


# Build release application
# Adding necessary package in Alpine
RUN apk update
RUN apk add pkgconfig openssl openssl-dev musl-dev
# Per https://github.com/rust-lang/rust/issues/40174, solution is implemented
#  on rust package 1.44.0-beta
RUN rustup toolchain install beta
RUN rustup default beta
# Buid *the* application
RUN cargo build --target x86_64-unknown-linux-musl --release

# Expose listening port for application
EXPOSE 8080

# Run the application
CMD ["target/release/restapiwebservice"]

# STAGE 2 is to have smallest image possible by including only necessary binary
# Use smallest base image
FROM scratch

# Copy application binary from STAGE 1 image to STAGE 2 image
COPY --from=builder /usr/src/rustwebservice/target/x86_64-unknown-linux-musl/release/restapiwebservice /main

# Expose listening port for application
EXPOSE 8080

# Run the application
CMD ["/main"]