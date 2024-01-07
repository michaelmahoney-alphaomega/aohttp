// TODO
// 1. Build a multithread TCP listener object

pub mod http;
pub mod router;
pub mod logger;

use std::net::TcpListener;
use std::io::Error;
use crate::router::Route;

pub fn create_http_server(address: &str) -> Result<TcpListener, Error> 
{
    let http_server = TcpListener::bind(address)?;
    return Ok(http_server)
}

pub fn run_http_server(address: &str, routes: &Vec<&Route>) -> Result<i8, Error>
{
    let server = create_http_server(address)?;

    for stream in server.incoming() 
    {
        match stream
        {
            Ok(stream) => router::router(stream, routes),
            Err(e) => 
            {
                let error = &mut format!("{:?}", e);
                println!("{}", error);
                logger::log(error, "logs/http_server.log").unwrap_err();
            }
        }
    }
    Ok(0)
}