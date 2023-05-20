# Build Stage
FROM rust:1.68.0 as builder

WORKDIR /granit
RUN USER=root cargo new --bin granit
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ADD ./ ./
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder granit/target/release/granit ${APP}/granit

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

EXPOSE 8080

CMD /usr/src/app/granit