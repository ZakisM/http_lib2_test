use http_lib2::{request::ServerRequest, response::HttpResponse, server::Server};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut server = Server::new([127, 0, 0, 1], 1234);

    server.at("/").get(hello_world);
    server.at("/vec_u8").get(vec_u8);
    server.at("/hello/{name}").get(hello_name);
    server.at("/hello/{name}/{age}").get(hello_name_age);
    server
        .at("/hello/{name}/test/{age}")
        .get(hello_name_test_age);

    server.start()?;

    Ok(())
}

fn hello_world(_: ServerRequest) -> impl HttpResponse {
    "Hello World!"
}

fn vec_u8(_: ServerRequest) -> impl HttpResponse {
    let hello_world_bytes = b"Hello World Bytes!";

    hello_world_bytes.to_vec()
}

fn hello_name(req: ServerRequest) -> impl HttpResponse {
    let name = req.path("name")?;

    let res = format!("Hello {}!", name);

    Ok(res)
}

fn hello_name_age(req: ServerRequest) -> impl HttpResponse {
    let name = req.path("name")?;
    let age = req.path("age")?;

    let res = format!("Hello {}, aged {}!", name, age);

    Ok(res)
}

fn hello_name_test_age(req: ServerRequest) -> impl HttpResponse {
    let name = req.path("name")?;
    let age = req.path("age")?;

    let res = format!("[Test] Hello {}, aged {}!", name, age);

    Ok(res)
}
