use anemo::{rpc::Status, Request, Response};
use serde::{Deserialize, Serialize};
use tracing::info;

pub use hello::{
    hello_client::HelloClient,
    hello_server::{Hello, HelloServer},
};

pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/chat.helloworld.Hello.rs"));
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloResponse {
    pub message: String,
}

#[derive(Default)]
pub struct MyHello {}

#[anemo::async_trait]
impl Hello for MyHello{
    async fn say_hello(
        &self,
        request: Request<HelloRequest>
    )-> Result<Response<HelloResponse>, Status>{
        info!(
            "Got request from {}",
            request.peer_id().unwrap().short_display(4)
        );

        let reply = HelloResponse {
            message: format!("Hello")
        };

        Ok(Response::new(reply))
    }
}