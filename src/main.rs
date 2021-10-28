use std::error::Error;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use http_lib2::header_item::HeaderItem;
use http_lib2::http_item::HttpItem;
use http_lib2::request::Request;
use http_lib2::response::ResponseBuilder;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;

    for stream in listener.incoming() {
        thread::spawn(move || {
            if let Err(e) = handle_connection(stream) {
                eprintln!("{}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(stream: std::result::Result<TcpStream, std::io::Error>) -> Result<()> {
    let stream = stream?;

    stream.set_nodelay(true)?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;

    let req_s = stream;
    let res_s = req_s.try_clone()?;

    let mut req_buf = BufReader::new(req_s);
    let mut res_buf = BufWriter::new(res_s);

    loop {
        match Request::from_stream(req_buf.by_ref()) {
            Ok(request) => {
                let req_headers = request.header.header_map();

                let response = ResponseBuilder::new().body(b"Hello Zak!").build();

                response.write_to(res_buf.by_ref())?;

                if !req_headers.contains_by_str_key_value("connection", "keep-alive")
                    || req_headers.contains_by_str_key_value("connection", "close")
                {
                    break;
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
