FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN rustup install nightly
RUN rustup default nightly

RUN rustup target add wasm32-unknown-unknown

RUN cargo +nightly install wasm-pack

RUN wasm-pack build --target web --out-dir ./pkg

FROM nginx:alpine
COPY --from=builder /app/pkg /usr/share/nginx/html/pkg
COPY index.html /usr/share/nginx/html
COPY style.css /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
