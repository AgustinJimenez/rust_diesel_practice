# Use the official Rust image as the base image
FROM rust:1.64.0

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Install any necessary dependencies
RUN cargo build

# Set the entrypoint to the Rust binary
CMD ["cargo", "run"]