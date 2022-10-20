# GRPC Web Client Example
This example shows a grpc server using tonic with grpc web support.

## Setup
1. prost_build needs the PROTOC environment variable to point to the [protoc compile](https://github.com/protocolbuffers/protobuf/releases/latest).

## Run
1. Run `cargo run`.

## Endpoints
* ipv6: [[::1]:50051]([::1]:50051)
* ipv4: [localhost:50051](localhost:50051)

## Build
1. Run `cargo build`.

## Generated Code
The generated code will be located at ./output.

#### Yew Docs
[Yew Docs](https://yew.rs/docs/next/getting-started/introduction)

### TLS?
TLS should work although I have not tested it. [See here](https://github.com/devashishdxt/tonic-web-wasm-client/pull/14/commits/28729e9bac3bfbfbbb0b89c6db32e208a8b80036).