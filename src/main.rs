use http_lib2::{response::ResponseBuilder, server::Server};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut server = Server::new([127, 0, 0, 1], 1234);

    server
        .at("/")
        .get(|_| ResponseBuilder::new().body(b"Hello World").build());

    server
        .at("/Zak")
        .get(|_| ResponseBuilder::new().body(b"Hello Zakis").build());

    server.start()?;

    Ok(())
}
