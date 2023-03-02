[![rust-version-badge][]][rust-version] [![build][]][build-url] [![codecov][]][codecov-url]  

# rchip8

A Chip8 interpreter with a WebAssembly front-end, written in Rust.

# Demo

A [live demo](https://blog.landhb.dev/rchip8) is on my blog.


# Building

Install `wasm-pack`:

```sh
cargo install wasm-pack
```

Build with:

```sh
cd wasm
wasm-pack build --target web
```

View by running a web server in the `wasm` directory:

```sh
python3 -m http.server 
```


[//]: # (badges)
[rust-version-badge]: https://img.shields.io/badge/rust-latest%20stable-blue.svg?style=flat-square
[rust-version]: #rust-version-policy

[codecov]: https://img.shields.io/codecov/c/github/landhb/rchip8?style=flat-square
[codecov-url]: https://codecov.io/gh/landhb/rchip8

[build]: https://img.shields.io/github/actions/workflow/status/landhb/rchip8/build.yml?branch=master&style=flat-square
[build-url]: https://github.com/landhb/rchip8/actions?query=workflow%3ABuild
