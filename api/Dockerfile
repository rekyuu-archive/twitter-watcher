FROM rust:1.57

RUN apt-get -y update && \
    apt-get -y upgrade && \
    apt-get -y install sqlite3 libsqlite3-dev

WORKDIR /usr/src/twitter-watcher-api
COPY . .

RUN cargo install --path .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=5080

CMD ["twitter-watcher-api"]