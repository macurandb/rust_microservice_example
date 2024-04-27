ENV APP webdocker
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP  /usr/local/bin/$APP

EXPOSE 8080
CMD ["webdocker"]