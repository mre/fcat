FROM ekidd/rust-musl-builder:1.27.2 as builder

# create a new empty shell project
RUN USER=rust cargo new --bin fcat
WORKDIR /home/rust/src/fcat

# copy over your manifests
COPY ./Cargo.lock Cargo.lock
COPY ./Cargo.toml Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
# remove cached build artifact to prevent caching issues
RUN rm target/x86_64-unknown-linux-musl/release/fcat

# copy & build source files
COPY src/ src/
RUN cargo build --release

FROM alpine:latest

RUN apk --update --no-cache add pv && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/fcat/target/x86_64-unknown-linux-musl/release/fcat /usr/local/sbin/fcat

COPY entrypoint /entrypoint
ENTRYPOINT ["/entrypoint"]
