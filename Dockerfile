FROM rust:1.51.0-slim AS BUILD_LAYER

RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /opt/rusty

ENV PKG_CONFIG_ALLOW_CROSS=1
COPY Cargo.toml ./
COPY src/ ./src/

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.13
WORKDIR /opt/rusty

RUN apk --no-cache add ca-certificates
COPY --from=BUILD_LAYER /opt/rusty/target/x86_64-unknown-linux-musl/release/actix-web-demo .

RUN adduser appuser -s nologin -DH
USER appuser
EXPOSE 8080
ENTRYPOINT ["sh", "-c", "/opt/rusty/actix-web-demo"]
