FROM rust:1.47.0-buster AS builder
WORKDIR /builder
RUN rustup toolchain install nightly && rustup default nightly
RUN apt-get update && apt-get install -y default-libmysqlclient-dev
RUN cargo install diesel_cli --no-default-features --features mysql
COPY . .
RUN cargo build --release

FROM rust:1.47.0-buster AS development
WORKDIR /app
VOLUME [ "/app" ]
ENV DATABASE_URL='mysql://root:secret@mysql:3306/easyadmin'
RUN rustup toolchain install nightly && rustup default nightly
RUN apt-get update && apt-get install default-libmysqlclient-dev
CMD [ "cargo", "run" ]

FROM rust:1.47.0-buster AS migrations
WORKDIR /app
ENV DATABASE_URL='mysql://root:secret@mysql:3306/easyadmin'
RUN rustup toolchain install nightly && rustup default nightly
COPY --from=builder /usr/local/cargo/bin/diesel diesel
CMD [ "diesel", "migration", "--migration-dir", "database/migration", "run" ]

FROM debian:buster AS production
WORKDIR /app
# For now just place secret here, remove it later or dynamically generate it.
# It is need for rocket production build server.
# To generate it, run: openssl rand -base64 32
ENV DATABASE_URL='mysql://root:secret@mysql:3306/easyadmin'
ENV ROCKET_SECRET_KEY=vKgfkbc7EIJIrvVe0vPDkFVMtITsiwiv0BmQzZP77E8=
# Mysql client adapter currently couldn't be linked statically
RUN apt-get update && apt-get install -y default-libmysqlclient-dev
COPY --from=builder /builder/target/release/easyadmin-app easyadmin
COPY --from=builder /builder/public public
COPY --from=builder /builder/views views
CMD [ "./easyadmin" ]