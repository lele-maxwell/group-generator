# Stage 1: Build the application
FROM rust:latest AS builder

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Update package list and install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a minimal image for the application
FROM debian:latest

# Update package list and install dependencies
RUN apt-get update && apt-get install -y libssl-dev

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/group-generator /app/

# Make the binary executable
RUN chmod +x /app/group-generator

# Set the command to run the application
CMD ["/app/group-generator"]
