extern crate serde_json; 

use std::{net::TcpStream,io::{prelude::*, BufReader, BufWriter, Error}};
use serde_json::{Value, json};
use crate::http::{HttpRequest, HttpResponse, ApiResource}; // local module

pub enum Handler {
    Route,

}

struct Route {
    request: HttpRequest,
    response: Option<HttpResponse>,
    handler_func: Option<fn(request: HttpRequest) -> HttpResponse>}


impl Route {
    fn new(request:HttpRequest, handle_func: fn(request: HttpRequest) -> HttpResponse) {   
        self.request = request;
        self.response = None;
        self.handler_func = handle_func;}}


fn collect_stream(tcp_stream_ref: &TcpStream) -> Value {
    let buf_reader =  BufReader::new(tcp_stream_ref);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line: &String| !line.is_empty())
        .collect();

    let request: Value = json!(request); //convert Vec to serde_json::Value
    return request}

pub fn find_route(http_ref: &HttpRequest) -> Result<Route, Error> {
    let api_resource = http_ref.uri;
}

pub fn router(tcp_stream: TcpStream) -> () {
    let http_request = match HttpRequest::build_from_stream(&tcp_stream) {
        Ok(http_request) => http_request,
        Err(e) => panic!("This is broken, here's the error: {e}")};
    

    let response = Route {request: http_request, response: None, handler_func: auth_handler()};

    let mut buf_writer = BufWriter::new(tcp_stream);
    buf_writer.write(&response).unwrap();

    // let response_body = "Hello, World!";
    // let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\n{response_body}");
    // let mut buf_writer = BufWriter::new(tcp_stream);
    // buf_writer.write(&response.as_bytes()).unwrap();
    // let response = serde_json::to_string(&response).unwrap();
    // println!("{response}");

    // let response = response.as_bytes();
    // serde_json::to_writer(stream, &response).unwrap();
    // buf_writer.flush().unwrap();
}