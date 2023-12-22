// TODO
// 1. Build a multithread TCP listener object

pub mod http;
pub mod router;
pub mod logger;

use std::net::TcpListener;
use std::io::Error;

pub fn create_http_server(address: &str) -> Result<TcpListener, Error> 
{
    let http_server = TcpListener::bind(address)?;
    return Ok(http_server)
}

pub fn run_http_server(address: &str) -> Result<i8, Error>
{
    let server = create_http_server(address)?;

    for stream in server.incoming() 
    {
        match stream
        {
            Ok(stream) => router::router(stream),
            Err(e) => 
            {
                let error_message = format!("{:?}", e);
                logger::log(&error_message, "http_server.log");
            }
        }
    }
    Ok(0)
}