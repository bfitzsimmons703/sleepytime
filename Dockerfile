# Rust as the base image
FROM rust:1.60 as build

# Create a new empty shell project
RUN USER=root cargo new --bin sleepytime
WORKDIR /sleepytime

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/sleepytime*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

ARG APP=/usr/src/app

ENV APP_USER=sleepytime

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copy from the previous build
COPY --from=build /sleepytime/target/release/sleepytime ${APP}/sleepytime

# Make sure our app user has the right permissions
RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

# Run the binary
CMD ["./sleepytime"]