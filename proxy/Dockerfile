FROM rust:slim-buster

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev build-essential libapr1-dev libssl-dev cmake pkg-config


WORKDIR /app
COPY . .

RUN cargo build

CMD ["echo", "Hello from Rust Dockerfile!"]