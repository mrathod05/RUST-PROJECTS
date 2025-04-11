# Use official Rust image
FROM rust:latest as builder

# Create a new directory for our application
WORKDIR /app

# Copy the project files
COPY . .

# Install dependencies and build the application
RUN cargo build --release

# Create a lightweight image
FROM debian:buster-slim

# Install OpenSSL and other dependencies
RUN apt-get update && apt-get install -y openssl ca-certificates

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust-postgres-crud /usr/local/bin/

# Set the environment
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Expose the port the app runs on
EXPOSE 8000

# Run the binary
CMD ["rust-postgres-crud"]