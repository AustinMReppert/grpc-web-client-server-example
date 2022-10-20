use tonic::{Request, Response, Status};

use test::greeter_server::{Greeter};
use test::{HelloReply, HelloRequest};

pub mod test {
  include!("../output/test.rs");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
  async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
    println!("Got a request: {:?}", request);
    let reply = test::HelloReply {  
      message: format!("Hello {}!", request.get_ref().name).into(),
    };

    Ok(Response::new(reply))
  }
}