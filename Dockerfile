FROM alpine:edge

RUN echo '@testing http://dl-cdn.alpinelinux.org/alpine/edge/testing' >> /etc/apk/repositories

RUN apk add --no-cache \
    cargo@testing \
    g++ \
    rust@testing

COPY . /rust/

WORKDIR /rust/

RUN cargo build --release

CMD ./target/release/iron_hello_world
