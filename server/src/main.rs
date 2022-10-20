use tonic::{transport::Server};

use my_greeter::test::greeter_server::{GreeterServer};
use my_greeter::MyGreeter;

mod my_greeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse().unwrap();
  let greeter = GreeterServer::new(MyGreeter::default());

  Server::builder()
    .accept_http1(true)
    .add_service(tonic_web::enable(greeter))
    .serve(addr)
    .await?;

  Ok(())
}