# Build stage
FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .

# Install OpenSSL development package
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/web-load-tester .
COPY --from=builder /usr/src/app/static ./static

# Expose the port
EXPOSE 8080

# Run the application
CMD ["./web-load-tester"] 