//TODO
// 1. 

extern crate serde_json; 

use std::{net::TcpStream,io::{prelude::*, BufReader, BufWriter, Error as ioError}};
use serde_json::{Value, json};
use crate::http::*; // local module

pub struct Error {
    error_code: ErrorKind, 
}

pub enum ErrorKind {
    // TODO
    // research what the standard is for propogating error in rust function calls. 
    RouteNotFound(),
}

pub struct Route {
    // TODO
    // 1. Additional validation
    // 2. some encryption?
    auth: HttpAuth,
    uri: Uri,
    handler: Option<fn(Route) -> HttpResponse>,
}

impl Route {
    fn find_route(uri: &Uri, routes: Vec<&str>) -> Result<Route, Error> {}
    fn execute_route(&self) -> Result<HttpResponse, Error> {
        let handler_func = self.handler.unwrap();
        // {
        //     Ok(func) => func.unwrap(),
        //     Err(E) => E
        // }
    }
}


// impl Route {
//     fn new(request:HttpRequest, handle_func: fn(request: HttpRequest) -> HttpResponse) {   
//         self.request = request;
//         self.response = None;
//         self.handler_func = handle_func;}}


fn collect_stream(tcp_stream_ref: &TcpStream) -> Value {
    let buf_reader =  BufReader::new(tcp_stream_ref);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line: &String| !line.is_empty())
        .collect();

    let request: Value = json!(request); //convert Vec to serde_json::Value
    return request}




// this is the main workhorse function of this crate\
// it's currently skeletoned out for development
pub fn router(tcp_stream: TcpStream) -> () {
    let http_request = match HttpRequest::build_from_stream(&tcp_stream) {
        Ok(http_request) => http_request,
        Err(e) => panic!("ERROR: Failed to build the HttpRequest opject from the TcpStream. Please see the inner error: {e}")};
    

    let route: Route = Route::find_route(&http_request); // implement find_route fn
    let response: HttpResponse = Route::handler()

    // let mut buf_writer = BufWriter::new(tcp_stream);
    // buf_writer.write(&response).unwrap();

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