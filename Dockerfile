FROM rust as rust_builder

WORKDIR /usr/src/

RUN USER=root cargo new --bin video-to-gif

WORKDIR /usr/src/video-to-gif

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm ./target/release/deps/video*
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release


FROM node:14 as svelte_builder

WORKDIR /usr/src/

COPY ./front ./

RUN yarn \
		&& yarn build
		
FROM debian:bookworm-slim

WORKDIR /usr/src/video-to-gif

RUN apt-get update \
		&& apt-get install libssl3

COPY --from=rust_builder /usr/src/video-to-gif/target/release/video-to-gif ./video-to-gif
COPY --from=svelte_builder /usr/src/build ./front/build

CMD ./video-to-gif
