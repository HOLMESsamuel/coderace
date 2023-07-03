pub fn write_rust_dockerfile(version : &str) -> String {
    let rust_dockerfile = format!(
        "# Use the official Rust image as the base image
FROM rust:{version}

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock file into the container
COPY Cargo.toml Cargo.lock ./

# Install the cargo dependencies
RUN cargo install --path .

# Copy the rest of the application code into the container
COPY . .

# Expose the port 80
EXPOSE 80

# Run the command to start the application
CMD [\"cargo\", \"run\"]", version = version);
    rust_dockerfile
}
