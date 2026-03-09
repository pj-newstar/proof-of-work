FROM rust:1-alpine AS builder

# Install build dependencies for musl target
RUN apk add --no-cache \
    musl-dev \
    gmp-dev \
    mpfr-dev \
    mpc1-dev \
    m4 \
    make \
    gcc

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build

# Copy cargo files
COPY Cargo.* ./

# Copy source code
COPY src ./src

# Build the release binary with mount cache
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/target \
    cargo build --release --target x86_64-unknown-linux-musl && \
    cp /build/target/x86_64-unknown-linux-musl/release/pow /usr/local/bin/pow

# Create final minimal image
FROM alpine:latest

# Copy the binary from builder
COPY --from=builder /usr/local/bin/pow /usr/local/bin/pow

# Set the binary as executable
RUN chmod +x /usr/local/bin/pow

ENTRYPOINT ["/usr/local/bin/pow"]
CMD ["--help"]
