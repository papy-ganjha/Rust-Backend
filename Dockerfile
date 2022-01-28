FROM rust

WORKDIR /app

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

RUN cargo install cargo-watch
# RUN rustup toolchain install nightly && rustup default nightly

# Run the binary
EXPOSE 7878
CMD cargo watch -x run