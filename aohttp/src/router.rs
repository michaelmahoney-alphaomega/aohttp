//TODO
// • error handling for no route
// • read in the routes prior to the router. You don't want a file read on each request handle

extern crate serde_json; 

use std::{fs::File, net::TcpStream,io::{prelude::*, BufReader, BufWriter}};
use serde_json::{Value, json};
use crate::http::*; // local module


#[derive(Debug)]
pub struct Error {
    error_code: ErrorKind, }


#[derive(Debug)]
pub enum ErrorKind {
    // TODO
    // research what the standard is for propogating error in rust function calls. 
    RouteNotFound,}


#[derive(Debug)]
pub struct Route {
    // TODO
    // 1. Additional validation
    // 2. some encryption?
    auth: HttpAuth,
    uri: Uri,
    handler: Option<fn(&Route) -> Result<HttpResponse, Error>>,}




pub fn read_in_routes() -> Vec<String> {
    let file = File::open("routes").expect("The routes file is missing from the root of the project.");
    let buf = BufReader::new(file);
    let routes: Vec<String> = buf.lines()
        .map(|x| x.expect("Failed to parse line in routes. Keep the characters UTF-8"))
        .take_while(|line: &String| !line.is_empty())
        .collect();

    return routes
}
    
impl Route {
    fn find_route(uri: &Uri, routes: Vec<Route>) -> Result<Route, Error> {
        let found = false;
        for route in routes {
            if uri.path == route.uri.path {
                found = true;
                return Ok(route)}

            else {
                continue;}

        if found == false {return None;}}}

    fn execute_route(&self) -> Result<HttpResponse, Error> {
        let http_response = match self.handler {
            Some(func) => {func(self)},
            None => panic!("Didn't find the route, need to actually use error kinds here I think.")};

        http_response}}


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
pub fn router(tcp_stream: TcpStream) {
    let http_request = match HttpRequest::build_from_stream(&tcp_stream) {
        Ok(http_request) => http_request,
        Err(e) => panic!("ERROR: Failed to build the HttpRequest opject from the TcpStream. Please see the inner error: {e}")};
    
    // read in routes from "routes"
    let file = File::open("routes").expect("The routes file is missing from the root of the project.");
    let buf = BufReader::new(file);
    let routes: Vec<String> = buf.lines()
        .map(|x| x.expect("Failed to parse line in routes. Keep the characters UTF-8"))
        .take_while(|line: &String| !line.is_empty())
        .collect();

    let route = match Route::find_route(&http_request.uri, routes){
        Ok(x) => x,
        Err(error) => panic!("{:?}", error)};

    let response = match Route::execute_route(&route){
        Ok(response) => response,
        Err(error) => panic!("{:?}", error)};

    let response_bytes: &[u8] = response.as_bytes();
    let mut buf_writer = BufWriter::new(tcp_stream);
    buf_writer.write(response_bytes).unwrap();
    buf_writer.flush();
}