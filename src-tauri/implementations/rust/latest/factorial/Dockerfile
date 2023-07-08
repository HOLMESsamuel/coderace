# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy files into the container
COPY . .

# Install the cargo dependencies
RUN cargo install --path .

# Expose the port 80
EXPOSE 80

# Run the command to start the application
CMD ["cargo", "run"]