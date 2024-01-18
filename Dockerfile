# Use a Rust runtime as a base image
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files to speed up the dependency resolution
COPY ./Cargo.toml ./Cargo.lock ./

# Build dependencies (dummy project to cache dependencies)
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy the entire project to the container
COPY . .

# Build the actual application
RUN cargo build --release

# Expose the port your application will run on
EXPOSE 3000

# Command to run your application
CMD ["./target/release/api"]
