FROM rust:1.57

WORKDIR /usr/src/twitter-watcher
COPY . .

RUN cargo install --path .

CMD ["twitter-watcher"]