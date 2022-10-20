# GRPC Web Client Example
This example shows a grpc web client using tonic and yew.

## Setup
1. Ensure the wasm32 target exists `rustup target add wasm32-unknown-unknown`.
2. prost_build needs the PROTOC environment variable to point to the [protoc compiler](https://github.com/protocolbuffers/protobuf/releases/latest).
3. Install trunk for bundling and serving yew. [Instructions here.](https://trunkrs.dev/)

## Run
1. Run `trunk serve`.
2. Open [http://127.0.0.1:8080/](http://127.0.0.1:8080/).

## Build
1. Run `cargo build --target wasm32-unknown-unknown`.

## Generated Code
The generated code will be located at ./output.

#### Yew Docs
[Yew Docs](https://yew.rs/docs/next/getting-started/introduction)

### TLS?
TLS should work although I have not tested it. [See here](https://github.com/devashishdxt/tonic-web-wasm-client/pull/14/commits/28729e9bac3bfbfbbb0b89c6db32e208a8b80036).