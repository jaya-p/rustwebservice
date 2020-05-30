# Build: docker build -t rustwebservice . 
#         && docker images
# Run: docker run -p 8080:8080 rustwebservice // docker run -p <host-port>:<container-port> <image-name>
# Test: curl http://localhost:8080/

# Use rust-based image for container; rustc version 1.43.1
FROM rust:1.43.1

# Set working directory in container
RUN mkdir /usr/src/rustwebservice
WORKDIR /usr/src/rustwebservice

# Copy all source code file from local computer to container
COPY src src
COPY Cargo.toml .
COPY LICENSE .

# Build release application
RUN cargo build --release

# Expose listening port for application
EXPOSE 8080

# Run the application
CMD ["target/release/rustwebservice"]