ARG APP_NAME=p4nimau-rust
FROM rust:latest AS build
ARG APP_NAME
WORKDIR /app
RUN apt update && apt -yq install ca-certificates
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=.env,target=.env \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

ADD https://github.com/coord-e/magicpak/releases/download/v1.4.0/magicpak-x86_64-unknown-linux-musl /usr/bin/magicpak
RUN chmod +x /usr/bin/magicpak
RUN /usr/bin/magicpak -v /bin/server /bundle

FROM scratch as final
COPY --from=0 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=0 /bundle /.
ENTRYPOINT ["/bin/server"]