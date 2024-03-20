/////////////////////////////////////////
// This module is a shell for all the enums and structs involving http operations
/////////////////////////////////////////

extern crate serde_json;
extern crate regex;

use std::{net::TcpStream,io::{prelude::*, BufReader, Error},};
use serde_json::{Value, json};
use regex::Regex;


//////////////////////////////////////// 
// Data Structures
////////////////////////////////////////
 
#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: Uri,
    pub protocol: HttpProtocol,
    pub auth: HttpAuth,
    pub headers: Value,
    pub body: Vec<u8>,}

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: HttpStatusCode,
    pub headers: Value,
    pub body: Vec<u8>,}

#[derive(Debug)]
pub enum HttpMethod {
    Get(String),
    Delete(String),
    Patch(String),
    Post(String),
    Put(String),
    Update(String)}

#[derive(Debug)]
pub enum HttpStatusCode {
    Ok200(u16),
    Ok201(u16),
    TempRedirect301(u16),
    PermRedirect307(u16),
    BadRequest401(u16),
    Unauth403(u16),
    NotFound404(u16),
    ServerError505(u16),}

#[derive(Debug)]
pub enum HttpProtocol {
    Http10(String),
    Http11(String),
    Http12(String),
    Http13(String),
    Http14(String),
    Http2(String)}

#[derive(Debug)]
pub enum HttpAuth {
    Basic(String),
    Modern(String),
    OAuth(String),
    OAuth2(String),
    BadAuth(String),
    NoAuth}

#[derive(Debug)]
pub struct Uri { 
    // TODO
    // 1. implement the to_* and from_* methods
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,}



fn parse_request_line<'a>(line: &String) -> Result<Vec<String>, Error> {
    let pattern = r"(?i)^(GET|POST|PUT|DELETE|HEAD|OPTIONS|TRACE|CONNECT)\s+([^\s?#]+)(?:\?([^\s#_]*))?(?:#([^\s_]*))?\s+HTTP/([0-9.]+)$";
    // Create a regex object from the pattern
    let re = Regex::new(pattern).unwrap();

    // Check if the request matches the regex
    let caps = re.captures(&line).expect("There was no string after the regex filter - likely poison HTTP request"); 
    let caps_len = caps.len();

    // Extract the request components from the capture groups 
    let processed_line = &caps[0];
    println!("processed line: {processed_line}");

    let method = caps.get(1).unwrap().as_str().to_string(); // The HTTP method
    println!("method: {method}");

    let path = caps.get(2).unwrap().as_str().to_string(); // The URI path
    println!("path: {path}");
    
    let query = match caps.get(3) {
        Some(query) => query.as_str().to_string(),
        _ => "".to_string()}; 
    println!("query: {query}");


    let fragment = match caps.get(4) 
    {
        Some(query) => query.as_str().to_string(),
        _ => "".to_string()
    }; 

    println!("fragment: {fragment}");

    let version = match caps.get(usize::MAX)
    {
        Some(version) => version.as_str().to_string(),
        _ => "".to_string()
    };
    println!("version: {version}");

    // Sanitize URI by removing any characters that are not alphanumeric, dash, dot, slash, or tilde
    // let re_sanitize = Regex::new(r"[^a-zA-Z0-9-./~]").unwrap();

    // let method = String::from(method);

    // let path = String::from(
    //     re_sanitize.replace_all(path, "").as_ref());

    // let query = String::from(
    //     re_sanitize.replace_all(query, "").as_ref());

    // let fragment = String::from(
    //     re_sanitize.replace_all(fragment, "").as_ref());

    // let version = String::from(version);

    let mut parsed_line = Vec::new();
    parsed_line.push(method); 
    parsed_line.push(path);
    parsed_line.push(query);
    parsed_line.push(fragment);
    parsed_line.push(version);
    
    return Ok(parsed_line)}

impl<'a> HttpRequest {
    pub fn build_from_stream<'b> (tcp_stream: &TcpStream) -> Result<HttpRequest, Error> {
        // stream to BufReader - limits system calls
        let buf_reader =  BufReader::new(tcp_stream);
        let request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        
        
        // pull values from the request, request link
        let request_line = &request[0];
        let mut request_headers = Vec::<&str>::new();
        let line_break = 0;

        for line in &request {
            let line = line.as_str();   

            if !line.is_empty() {
                request_headers.push(line)}
            else {
                let _line_break: usize = request.iter().position(|_line| true).unwrap();
                break;}}


        // everything above break line convert to json=headers, all below is body      
        let request_headers = json!(request_headers);
        // println!("{:?}", request_headers);
        let request_body= request[line_break + 1..].concat().into_bytes();
        // let request_body = re cquest_body.concat().as_bytes();
        
        let parsed_line = parse_request_line(request_line).unwrap();
        for vec in &parsed_line
        {
            println!("vec: {:?}", vec);

        }
        let method = match parsed_line.get(0) {
            Some(x) => x.to_owned(),
            _ => panic!("ERROR: The request was a valid http request. Killing the stream")};

        let path = match parsed_line.get(1) {
            Some(x) => x.to_owned(),
            _ => panic!("ERROR: The request was a valid http request. Killing the stream")};

        let query = match parsed_line.get(2) {
            Some(x) => Some(x.to_owned()),
            _ => None};

        let fragment = match parsed_line.get(3) {
            Some(x) => Some(x.to_owned()),
            _ => None};

        let version = match parsed_line.get(4) {
            Some(x) => x.to_owned(),
            _ => panic!("ERROR: The request was a valid http request. Killing the stream")};

        // a big old regex string to separate the top line components and sanitize any poison characters in the requested URI
        
        
        // type the request
        let request_method = match method.as_str() {
            "GET"    => HttpMethod::Get(method),
            "Delete" => HttpMethod::Delete(method),
            "PATCH"  => HttpMethod::Patch(method),
            "POST"   => HttpMethod::Post(method),
            "PUT"    => HttpMethod::Put(method),
            "UPDATE" => HttpMethod::Update(method),
            &_       => HttpMethod::Get(method)};
        
        // type the requested resource based on the root element
        let request_uri = Uri {
            path: path,
            query: query,
            fragment: fragment
        };

        // type the protocol
        let request_protocol = match version.as_str() {
            "HTTP/1.0"   => HttpProtocol::Http10(version),
            "HTTP/1.1"   => HttpProtocol::Http11(version),
            "HTTP/2.0"   => HttpProtocol::Http2(version),
            &_          => HttpProtocol::Http11(version)
        };

        pub fn get_auth_type(request_headers: &Value) -> Result<HttpAuth,Error> {
            let custom_error = Error::new(std::io::ErrorKind::InvalidData, "Failed");
            let request_auth = match request_headers.get("Authorization"){
                Some(auth) => auth.to_string(),
                _ => String::from("")};
            
            if request_auth.contains("Basic") {
                return Ok(HttpAuth::Basic(request_auth))} //just do a lifetime for this
    
            else if request_auth.contains("Token") {
                return Ok(HttpAuth::Modern(request_auth))}//just do a lifetime for this
            
            else {
                return Ok(HttpAuth::NoAuth)}}

        let request_auth = get_auth_type(&request_headers).unwrap();      
    
        let http_request = HttpRequest {
            method: request_method,
            uri: request_uri,
            protocol: request_protocol,
            auth: request_auth, //need to define function that returns the right type
            headers: request_headers,
            body: request_body,}; // need to define function that return the right type

        return Ok(http_request)
    }
}

impl<'a> HttpRequest {
    pub fn as_bytes(&self) -> &[u8] {&[0;8]}
}

impl HttpResponse {
    pub fn as_bytes(&self) -> &[u8] {&[0;8]}
}