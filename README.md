# Convert videos to GIFs
### Svelte application for converting videofiles to gifs by using the WASM port of ffmpeg

* Navigate inside of `front/`
* Install dev dependencies required for building by running `yarn`
* Build production build of the Svelte project with `yarn build`

### Minimal webserver written in Rust using actix-web

* Build webserver executable with `cargo build --release`
* After building the executable can be found under `target/release`
---
* Server settings can be configured by creating `config.toml` at project root  
```
port = <number between 0 and 65535 | defaults to 8080>
ip = <string, should be set to your server's local ip | defaults to localhost>
worker_count = <usize, the amount of cores for actix to use | defaults to 2>
shutdown_timeout = <u64, soft shutdown timer in seconds | defaults to 15>
```