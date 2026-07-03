# rust >= 1.82 emits post-MVP wasm features that webpack 4 cannot parse
FROM rust:1.81 AS wasm
RUN curl -sSfL https://github.com/rustwasm/wasm-pack/releases/download/v0.13.1/wasm-pack-v0.13.1-x86_64-unknown-linux-musl.tar.gz \
    | tar xz --strip-components=1 -C /usr/local/bin wasm-pack-v0.13.1-x86_64-unknown-linux-musl/wasm-pack
WORKDIR /repo
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN wasm-pack build --release

FROM node:16-alpine AS build
WORKDIR /repo
COPY --from=wasm /repo/pkg pkg
COPY www www
WORKDIR /repo/www
RUN npm ci && npm run build

FROM nginx:alpine
COPY --from=build /repo/www/dist /usr/share/nginx/html
