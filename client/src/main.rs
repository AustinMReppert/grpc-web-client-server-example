use yew::prelude::*;
use yew::suspense::{use_future};
use tonic_web_wasm_client::Client;

use test::HelloRequest;

pub mod test {
  include!("../output/test.rs");
}

#[function_component]
fn App() -> Html {
  let suspension_result = use_future(|| {
    async {
      let client = Client::new("http://localhost:50051".to_string());

      let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
      });

      let mut greeter_client = test::greeter_client::GreeterClient::new(client);
      greeter_client.say_hello(request).await
    }
  });

  let response_html = match suspension_result {
    Ok(future_handle) => {
      match &*future_handle {
        Ok(response) => {
          html! {&response.get_ref().message}
        }
        Err(_) => {
          html! {"Error!"}
        }
      }
    }
    Err(_) => {
      html! {"In progress!"}
    }
  };

  html! {
    <main>
      <p>{ response_html }</p>
    </main>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}