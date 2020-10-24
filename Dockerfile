FROM rust:1.47.0-buster AS builder
WORKDIR /builder
RUN rustup toolchain install nightly && rustup default nightly
RUN apt-get update && apt-get install -y default-libmysqlclient-dev
RUN cargo install diesel_cli --no-default-features --features mysql
COPY . .
RUN cd cli && cargo build --release
RUN cargo build --release

FROM rust:1.47.0-buster AS development
WORKDIR /app
VOLUME [ "/app" ]
ENV DATABASE_URL=
ENV ROCKET_SECRET_KEY=
RUN rustup toolchain install nightly && rustup default nightly
RUN apt-get update && apt-get install default-libmysqlclient-dev
COPY --from=builder /builder/cli/target/release/easyadmin /usr/local/bin/easyadmin
CMD [ "cargo", "run" ]

FROM rust:1.47.0-buster AS migrations
WORKDIR /app
ENV DATABASE_URL=
ENV ROCKET_SECRET_KEY=
RUN rustup toolchain install nightly && rustup default nightly
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /builder/cli/target/release/easyadmin /usr/local/bin/easyadmin
CMD [ "easyadmin", "run:migrations" ]

FROM debian:buster AS production
WORKDIR /app
ENV DATABASE_URL=
ENV ROCKET_SECRET_KEY=
# Mysql client adapter currently couldn't be linked statically
RUN apt-get update && apt-get install -y default-libmysqlclient-dev
COPY --from=builder /builder/target/release/easyadmin-app easyadmin-app
COPY --from=builder /builder/public public
COPY --from=builder /builder/views views
CMD [ "./easyadmin-app" ]