# Use the latest Rust image as the base image
FROM rust:latest as builder

WORKDIR /app
COPY . .

# Install the nightly toolchain and set it as the default
RUN rustup install nightly
RUN rustup default nightly

# Add the WebAssembly target
RUN rustup target add wasm32-unknown-unknown

# Install wasm-pack with the nightly toolchain
RUN cargo +nightly install wasm-pack

# Build the project using wasm-pack
RUN wasm-pack build --target web --out-dir ./pkg

# Use a simple HTTP server to serve the static files
FROM nginx:alpine
COPY --from=builder /app/pkg /usr/share/nginx/html/pkg
COPY index.html /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
