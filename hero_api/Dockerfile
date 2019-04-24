FROM rustlang/rust:nightly as build
ARG PROJECT_NAME=hero_api
# create a new empty shell project
RUN USER=root cargo new --bin ${PROJECT_NAME}
WORKDIR /${PROJECT_NAME}
# fetch wait-for-it.sh
RUN curl https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh -o wait-for-it.sh && chmod +x wait-for-it.sh
RUN cargo install diesel_cli --no-default-features --features postgres
# copy dependency files
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# pre-build dependencies and cache result in docker layer
RUN cargo build
RUN rm src/*.rs
# copy your source tree
COPY ./src ./src
# build for release
RUN rm ./target/debug/deps/${PROJECT_NAME}*
RUN cargo build

#

FROM rust:1.30.1-slim
# RUN set -ex; apt-get update; apt-get install -y libpq-dev
ARG PROJECT_NAME=hero_api
# copy init dependencies
COPY --from=build /usr/lib/x86_64-linux-gnu /usr/lib/x86_64-linux-gnu
COPY --from=build /lib/x86_64-linux-gnu /lib/x86_64-linux-gnu
COPY --from=build /usr/local/cargo/bin/diesel ./bin/diesel
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations ./migrations

COPY --from=build /${PROJECT_NAME}/wait-for-it.sh .
# copy the build artifact from the build stage
COPY --from=build /${PROJECT_NAME}/target/debug/${PROJECT_NAME} ./bin/${PROJECT_NAME}
# # set the startup command to run your binary
# CMD ["./bin/${PROJECT_NAME}"]