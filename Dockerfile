FROM rust:1.27.2

RUN apt update \
    && apt install -y pv pipebench \
    && apt clean

# create a new empty shell project
RUN USER=root cargo new --bin fcat
WORKDIR /fcat
# set modification date in the past so the actual source files will be compiled
RUN touch -t 197001010000 src/main.rs

# copy over your manifests
COPY ./Cargo.lock Cargo.lock
COPY ./Cargo.toml Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY . .
RUN cargo install

COPY entrypoint /entrypoint
ENTRYPOINT ["/entrypoint"]
