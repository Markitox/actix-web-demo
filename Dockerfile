FROM public.ecr.aws/docker/library/rust:1.88-alpine AS build_layer

RUN apk update && apk add ca-certificates musl-dev && apk cache clean
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /opt/rusty
RUN adduser appuser -s nologin -DH

ENV PKG_CONFIG_ALLOW_CROSS=1
COPY Cargo.toml ./
COPY src/ ./src/

RUN cargo build --release

FROM scratch
WORKDIR /opt/rusty

COPY --from=build_layer /opt/rusty/target/release/actix-web-demo .
COPY --from=build_layer /etc/passwd /etc/passwd
USER appuser

EXPOSE 8080
ENTRYPOINT ["/opt/rusty/actix-web-demo"]
