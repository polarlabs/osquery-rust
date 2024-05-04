# Stage 1: build dependencies
FROM lukemathwalker/cargo-chef:latest-rust-latest as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Runtime stage
FROM polarlabs/osquery:5.2.2-debian-stable-slim AS runtime

ENV OSQUERY_EXTENSION_PERMS=744
ENV OSQUERY_EXTENSION_NAME=table-proc-meminfo
ENV OSQUERY_EXTENSION_FOLDER=/data/osquery/ext
ENV OSQUERY_COMPILATION_TARGET=x86_64-unknown-linux-musl

WORKDIR /app
COPY --from=builder /app/target/$OSQUERY_COMPILATION_TARGET/debug/$OSQUERY_EXTENSION_NAME $OSQUERY_EXTENSION_FOLDER/

USER root
RUN chmod $OSQUERY_EXTENSION_PERMS $OSQUERY_EXTENSION_FOLDER/$OSQUERY_EXTENSION_NAME

WORKDIR $OSQUERY_EXTENSION_FOLDER
#ENTRYPOINT ["osqueryi", "--extension", "${OSQUERY_EXTENSION_NAME}"]
#CMD ["osqueryi", "--extension", "/tmp/table-proc-meminfo"]
CMD osqueryi --extension $OSQUERY_EXTENSION_FOLDER/$OSQUERY_EXTENSION_NAME
