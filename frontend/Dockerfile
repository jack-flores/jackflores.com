# install rust and dependencies
FROM rust:latest AS builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config curl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

# build
COPY . .
RUN trunk build --release

# confirm all files are readable
RUN chmod -R 755 dist

# static serve with nginx
FROM nginx:alpine
RUN rm -rf /usr/share/nginx/html/*
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /dist /usr/share/nginx/html
