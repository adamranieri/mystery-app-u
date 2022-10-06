FROM rust:1.31


COPY . /workspace
WORKDIR /workspace

RUN cargo install --path .
RUN carg build

ENTRYPOINT ["./target/debug/mystery-app-u"]