extern crate serde_json; 

use std::{
    net::TcpStream,
    io::{prelude::*, BufReader}};

use serde_json::{Value, json};

// use crate::http;

fn collect_stream(tcp_stream_ref: &TcpStream) -> Value {
    let buf_reader =  BufReader::new(tcp_stream_ref);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line: &String| !line.is_empty())
        .collect();

    let request: Value = json!(request); //convert Vec to serde_json::Value
    return request}

pub fn router(tcp_stream: TcpStream) -> () {
    // let http_request = match HttpRequest::build_from_stream(&tcp_stream) {
    //     Ok(http_request) => http_request,
    //     Err(e) => panic!("This is broken, here's the error: {e}")};
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