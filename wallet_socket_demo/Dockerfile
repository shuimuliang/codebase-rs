# Use the official Rust image as the base image
FROM rust:1.71

# Set the working directory in the Docker container
WORKDIR /usr/src/myapp

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Compile the application
RUN cargo build --release

# Expose port 7878
EXPOSE 7878

# Set the startup command to run the binary
CMD ["./target/release/wallet_socket_demo"]
