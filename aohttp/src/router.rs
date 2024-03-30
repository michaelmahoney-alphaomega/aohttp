//TODO
// • error handling for no route
// • read in the routes prior to the router. You don't want a file read on each request handle

extern crate serde_json; 

use std::{net::TcpStream,io::{prelude::*, BufWriter}};
use serde_json::json;
use crate::{http::*, logger}; // local module

#[derive(Debug)]
pub struct Error 
{
    error_code: ErrorKind, 
}

#[derive(Debug)]
pub enum ErrorKind 
{
    // TODO
    // research what the standard is for propogating error in rust function calls. 
    RouteNotFound,
    RouteHandlerNotFound,

}

#[derive(Debug)]
pub struct Route 
{
    // TODO
    // 1. Additional validation
    // 2. some encryption?
    pub auth: HttpAuth,
    pub uri: Uri,
    pub handler: Option<fn(&Route) -> Result<HttpResponse, Error>>,
}
    
impl Route {
    fn find_route<'a>(uri: &Uri, routes: &Vec<&'a Route>) -> Result<&'a Route, Error> {
        let mut answer: Result<&Route, Error> = Err(Error{error_code: ErrorKind::RouteNotFound});
        
        if routes.is_empty() 
        {
            println!("There are no routes in the router.");
            return answer
        }
        
        else 
        {
            for route in routes 
            {
                if uri.path == route.uri.path {
                    println!("Found the route: {:?}", route.uri.path);
                    answer = Ok(route);
                    break}

                else {continue;}
            }
            return answer
        }}

    fn execute_route(&self) -> Result<HttpResponse, Error> {
        let http_response = match self.handler 
        {
            Some(func) => {func(self)},
            None => Err(Error{error_code: ErrorKind::RouteHandlerNotFound})
        };
        http_response}}


// this is the main workhorse function of this crate\
// it's currently skeletoned out for development
pub fn router(tcp_stream: TcpStream, routes: &Vec<&Route>) 
{
    let root_route :Route = Route 
    { // NEED TO PUT THIS DETAILS IN A CONFIG FILE
        auth: HttpAuth::NoAuth,
        uri: Uri 
        {
            path: String::from("/"),
            query: None,
            fragment: None
        },

        handler: Some(|_route| -> Result<HttpResponse, Error> 
        {
            let response: HttpResponse 
                = HttpResponse 
                {
                    status_code: HttpStatusCode::NotFound404,
                    headers: json!({"Content-Type": "text/html"}),
                    body: Some("hello world!".as_bytes().to_vec())
                };

            return Ok(response)
        })
    };

    let http_request = match HttpRequest::build_from_stream(&tcp_stream) 
    {
        Ok(http_request) => http_request,
        Err(e) => panic!("ERROR: Failed to build the HttpRequest opject from the TcpStream. Please see the inner error: {e}")
    };

    let route = Route::find_route(&http_request.uri, routes)
        .unwrap_or_else(
            |error| 
            { 
                let mut message = format!("ERROR: {:?}", error.error_code);
                logger::log(&mut message, "logs/router.log").unwrap(); 
                return &root_route;
            });

    let response = Route::execute_route(&route).unwrap_or_else(
        |error| 
        {
            let mut message = format!("ERROR: {:?}", error.error_code);
            logger::log(&mut message, "logs/router.log").unwrap(); 
            return HttpResponse 
            {
                status_code: HttpStatusCode::InternalServerError500,
                headers: json!({"Content-Type": "text/html"}),
                body: None
            };
        });
    
    if response.body.is_some()
    {   
        let content_length = response.body.as_ref().unwrap().len();
        let mut message = format!("INFO response_code:{:?} response_length:{:?} request_uri:{:?} ", response.status_code, content_length, http_request.uri.path);
        logger::log(&mut message, "logs/router.log").unwrap();
    }

    else
    {
        let mut message = format!("INFO response_code:{:?} response_length:0 request_uri:{:?} ", response.status_code, http_request.uri.path);
        logger::log(&mut message, "logs/router.log").unwrap();
    }

    let response_bytes: &[u8] = &response.as_bytes();
    let mut buf_writer = BufWriter::new(tcp_stream);
    buf_writer.write(response_bytes).unwrap_or_else(
        |e| 
        {
            let mut message = format!("ERROR: {:?}", e);
            logger::log(&mut message, "logs/router.log").unwrap();
            return 0
        }
    );
    
    buf_writer.flush().unwrap_or_else(
        |e| 
        {
            let mut message = format!("ERROR: {:?}", e);
            logger::log(&mut message, "logs/router.log").unwrap();
        }
    );
}