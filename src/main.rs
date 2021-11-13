use http_lib2::{request::Request, response::HttpResponse, server::Server};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut server = Server::new([127, 0, 0, 1], 1234);

    server.at("/").get(hello_world);
    server.at("/vec_u8").get(vec_u8);

    server.start()?;

    Ok(())
}

fn hello_world(req: Request) -> impl HttpResponse {
    "Hello World!"
}

fn vec_u8(req: Request) -> impl HttpResponse {
    let hello_world_bytes = b"Hello World Bytes!";

    hello_world_bytes.to_vec()
}
