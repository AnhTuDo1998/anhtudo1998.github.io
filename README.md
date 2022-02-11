# Overview
A static resume website build totally by Rustlang. Frontend framework used is [Yew](https://yew.rs/), a reactive framework with similar principles to React.js

# Build
Website is written Rust which is then compiled to wasm32 target. The build toolchain we are using is [trunk](https://trunkrs.dev/). To install the toolchain (assume Rust is installed):

```
$ rustup target add wasm32-unknown-unknown
$ cargo install trunk
```

Then to build and open server (with auto reloading):

```
$ trunk serve
```