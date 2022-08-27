FROM rust:1.63 as builder
WORKDIR /usr/src/sat-recruiting-task
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/sat-recruiting-task/target/release ./app
CMD ["./app/sat-recruiting-task"]