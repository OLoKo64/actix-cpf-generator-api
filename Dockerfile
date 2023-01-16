# Use the official Rust image to create a build.
FROM rust as builder

# Copy local code to the container image.
COPY . /app

# Set the working directory.
WORKDIR /app

# Build the release with cargo build, including optimizations.
RUN cargo build --release

# Use the official Debian slim image for a lean production container.
FROM gcr.io/distroless/cc-debian11

# Copy the binary to the production image from the builder stage.
COPY --from=builder /app/target/release/cpf-generator-api /app/cpf-generator-api
WORKDIR /app

# Run the web service on container startup.
CMD ["./cpf-generator-api"]
