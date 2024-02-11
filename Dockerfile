FROM rust:latest 
WORKDIR /usr/src/app
COPY . .

# RUN cargo install .
RUN cargo build --release

# CMD ["cargo","run"]
CMD ["./target/release/calculator"]

