####################################################################################################
## Builder
####################################################################################################
# Use the official Rust image for building the application
FROM rust:1.81.0-bullseye AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

####################################################################################################
## Runtime
####################################################################################################
FROM debian:bullseye-slim

# Install necessary packages in the runtime base
RUN apt-get update && apt-get -y install ca-certificates && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/blockchain-rest-api-server .

# Copy the environment configuration files from the builder stage
COPY --from=builder /usr/src/app/src/environment ./src/environment
COPY --from=builder /usr/src/app/geth ./geth

# Expose port via environment variable, defaults to 3000 if not set
ENV PORT=3000

# Expose the port
EXPOSE ${PORT}

# Set the command to run your application
CMD ["./blockchain-rest-api-server"]
