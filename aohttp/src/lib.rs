// TODO
// 1. Build a multithread TCP listener object

use std::net::TcpListener;
use std::io::Error;

pub fn create_http_server(address: &str) -> Result<TcpListener, Error> 
{
    let http_server = TcpListener::bind(address)?;
    return Ok(http_server)
}

pub fn http_server () -> Result<HttpServer, Error>
{
    Ok(http_server)
}