FROM rust:1.47.0-alpine3.12 AS builder
WORKDIR /builder
RUN rustup toolchain install nightly && rustup default nightly
RUN apk add --no-cache musl-dev mariadb-dev
COPY . .
RUN cargo build --release

FROM alpine:3.12
WORKDIR /app
# For now just place secret here, remove it later or dynamically generate it.
# It is need for rocket production build server.
# To generate it, run: openssl rand -base64 32
ENV ROCKET_SECRET_KEY=vKgfkbc7EIJIrvVe0vPDkFVMtITsiwiv0BmQzZP77E8=
COPY --from=builder /builder/target/release/easyadmin-app easyadmin
COPY --from=builder /builder/public public
COPY --from=builder /builder/views views
CMD [ "./easyadmin" ]