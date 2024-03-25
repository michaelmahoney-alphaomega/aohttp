//TODO
// • error handling for no route
// • read in the routes prior to the router. You don't want a file read on each request handle

extern crate serde_json; 

use std::{net::TcpStream,io::{prelude::*, BufReader, BufWriter}};
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
    pub auth: HttpAuth,
    pub uri: Uri,
    pub handler: Option<fn(&Route) -> Result<HttpResponse, Error>>,}
    
impl Route {
    fn find_route<'a>(uri: &Uri, routes: &Vec<&'a Route>) -> Result<&'a Route, Error> {
        let mut answer: Result<&Route, Error> = Err(Error{error_code: ErrorKind::RouteNotFound});
        if routes.is_empty() {
            println!("There are no routes in the router.");
            return answer}
        
        else {
            for route in routes {
                if uri.path == route.uri.path {
                    println!("Found the route: {:?}", route.uri.path);
                    answer = Ok(route);
                    break}

                else {continue;}}

            return answer}}

    fn execute_route(&self) -> Result<HttpResponse, Error> {
        let http_response = match self.handler {
            Some(func) => {func(self)},
            None => panic!("ERROR: The route has no handler. Killing the stream.")};

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
pub fn router(tcp_stream: TcpStream, routes: &Vec<&Route>) {
    let root_route :Route = Route {
        auth: HttpAuth::Basic(String::from("No Auth Provided")),
        uri: Uri {
            path: String::from("/"),
            query: None,
            fragment: None},

        handler: Some(|route| -> Result<HttpResponse, Error> {
            let response = HttpResponse {
                status_code: HttpStatusCode::Ok200(200),
                headers: json!({"Content-Type": "text/html"}),
                body: Some("hello world!".as_bytes().to_vec())};

            return Ok(response)})};

    let http_request = match HttpRequest::build_from_stream(&tcp_stream) {
        Ok(http_request) => http_request,
        Err(e) => panic!("ERROR: Failed to build the HttpRequest opject from the TcpStream. Please see the inner error: {e}")};
    
    let route = Route::find_route(&http_request.uri, routes)
        .unwrap_or_else(|error| { 
            println!("{:?}", error); 
            return &root_route;});

    // let route = match Route::find_route(&http_request.uri, routes){
    //     Ok(x) => x,
    //     Err(error) => };

    let response = match Route::execute_route(&route) {
        Ok(response) => response,
        Err(error) => panic!("{:?}", error)};

    let response_bytes: &[u8] = &response.as_bytes();
    let mut buf_writer = BufWriter::new(tcp_stream);
    buf_writer.write(response_bytes).unwrap();
    buf_writer.flush().unwrap();
}