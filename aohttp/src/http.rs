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
    pub body: Option<Vec<u8>>,}

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Head,
    Options,
    Trace,
    Connect,
    Delete,
    Patch,
    Post,
    Put,
    Update}

#[derive(Debug)]
pub enum HttpStatusCode {
    Continue100,
    SwitchProtocols101,
    Processing102,
    EarlyHints103,
    Ok200,
    Created201,
    Accepted202,
    NonAuthoritativeInfo203,
    NoContent204,
    ResetContent205,
    PartialContent206,
    MultiStatus207,
    AlreadyReported208,
    ImUsed226,
    MultipleChoices300,
    MovedPermanently301,
    Found302,
    SeeOther303,
    NotModified304,
    UseProxy305,
    Unused306,
    TemporaryRedirect307,
    PermanentRedirect308,
    BadRequest400,
    Unauthorized401,
    PaymentRequired402,
    Forbidden403,
    NotFound404,
    MethodNotAllowed405,
    NotAcceptable406,
    ProxyAuthRequired407,
    RequestTimeout408,
    Conflict409,
    Gone410,
    LengthRequired411,
    PreconditionFailed412,
    PayloadTooLarge413,
    UriTooLong414,
    UnsupportedMediaType415,
    RangeNotSatisfiable416,
    ExpectationFailed417,
    ImATeapot418,
    MisdirectedRequest421,
    UnprocessableEntity422,
    Locked423,
    FailedDependency424,
    TooEarly425,
    UpgradeRequired426,
    PreconditionRequired428,
    TooManyRequests429,
    RequestHeaderFieldsTooLarge431,
    UnavailableForLegalReasons451,
    InternalServerError500,
    NotImplemented501,
    BadGateway502,
    ServiceUnavailable503,
    GatewayTimeout504,
    HttpVersionNotSupported505,
    VariantAlsoNegotiates506,
    InsufficientStorage507,
    LoopDetected508,
    NotExtended510,
    NetworkAuthRequired511,
}

#[derive(Debug)]
pub enum HttpProtocol {
    Http09,
    Http10,
    Http11,
    Http2,
    Http3
}

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
        for vec in &parsed_line{
            println!("vec: {:?}", vec);}
            
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
            "GET"    => HttpMethod::Get,
            "HEAD"   => HttpMethod::Head,
            "OPTIONS"=> HttpMethod::Options,
            "TRACE"  => HttpMethod::Trace,
            "CONNECT"=> HttpMethod::Connect,
            "Delete" => HttpMethod::Delete,
            "PATCH"  => HttpMethod::Patch,
            "POST"   => HttpMethod::Post,
            "PUT"    => HttpMethod::Put,
            "UPDATE" => HttpMethod::Update,
            &_       => HttpMethod::Get};
        
        // type the requested resource based on the root element
        let request_uri = Uri {
            path: path,
            query: query,
            fragment: fragment
        };

        // type the protocol
        let request_protocol = match version.as_str() {
            "HTTP/1.0"   => HttpProtocol::Http10,
            "HTTP/1.1"   => HttpProtocol::Http11,
            "HTTP/2.0"   => HttpProtocol::Http2,
            "HTTP/3.0"   => HttpProtocol::Http3,
            &_          => HttpProtocol::Http11,
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
    pub fn as_bytes(self) -> Vec<u8> {
        let mut response = Vec::new();
        let status_code = match self.status_code {
            HttpStatusCode::Continue100 => "HTTP/1.1 100 Continue",
            HttpStatusCode::SwitchProtocols101 => "HTTP/1.1 101 Switching Protocols",
            HttpStatusCode::Processing102 => "HTTP/1.1 102 Processing",
            HttpStatusCode::EarlyHints103 => "HTTP/1.1 103 Early Hints",
            HttpStatusCode::Ok200 => "HTTP/1.1 200 OK",
            HttpStatusCode::Created201 => "HTTP/1.1 201 Created",
            HttpStatusCode::Accepted202 => "HTTP/1.1 202 Accepted",
            HttpStatusCode::NonAuthoritativeInfo203 => "HTTP/1.1 203 Non-Authoritative Information",
            HttpStatusCode::NoContent204 => "HTTP/1.1 204 No Content",
            HttpStatusCode::ResetContent205 => "HTTP/1.1 205 Reset Content",
            HttpStatusCode::PartialContent206 => "HTTP/1.1 206 Partial Content",
            HttpStatusCode::MultiStatus207 => "HTTP/1.1 207 Multi-Status",
            HttpStatusCode::AlreadyReported208 => "HTTP/1.1 208 Already Reported",
            HttpStatusCode::ImUsed226 => "HTTP/1.1 226 IM Used",
            HttpStatusCode::MultipleChoices300 => "HTTP/1.1 300 Multiple Choices",
            HttpStatusCode::MovedPermanently301 => "HTTP/1.1 301 Moved Permanently",
            HttpStatusCode::Found302 => "HTTP/1.1 302 Found",
            HttpStatusCode::SeeOther303 => "HTTP/1.1 303 See Other",
            HttpStatusCode::NotModified304 => "HTTP/1.1 304 Not Modified",
            HttpStatusCode::UseProxy305 => "HTTP/1.1 305 Use Proxy",
            HttpStatusCode::Unused306 => "HTTP/1.1 306 Unused",
            HttpStatusCode::TemporaryRedirect307 => "HTTP/1.1 307 Temporary Redirect",
            HttpStatusCode::PermanentRedirect308 => "HTTP/1.1 308 Permanent Redirect",
            HttpStatusCode::BadRequest400 => "HTTP/1.1 400 Bad Request",
            HttpStatusCode::Unauthorized401 => "HTTP/1.1 401 Unauthorized",
            HttpStatusCode::PaymentRequired402 => "HTTP/1.1 402 Payment Required",
            HttpStatusCode::Forbidden403 => "HTTP/1.1 403 Forbidden",
            HttpStatusCode::NotFound404 => "HTTP/1.1 404 Not Found",
            HttpStatusCode::MethodNotAllowed405 => "HTTP/1.1 405 Method Not Allowed",
            HttpStatusCode::NotAcceptable406 => "HTTP/1.1 406 Not Acceptable",
            HttpStatusCode::ProxyAuthRequired407 => "HTTP/1.1 407 Proxy Authentication Required",
            HttpStatusCode::RequestTimeout408 => "HTTP/1.1 408 Request Timeout",
            HttpStatusCode::Conflict409 => "HTTP/1.1 409 Conflict",
            HttpStatusCode::Gone410 => "HTTP/1.1 410 Gone",
            HttpStatusCode::LengthRequired411 => "HTTP/1.1 411 Length Required",
            HttpStatusCode::PreconditionFailed412 => "HTTP/1.1 412 Precondition Failed",
            HttpStatusCode::PayloadTooLarge413 => "HTTP/1.1 413 Payload Too Large",
            HttpStatusCode::UriTooLong414 => "HTTP/1.1 414 URI Too Long",
            HttpStatusCode::UnsupportedMediaType415 => "HTTP/1.1 415 Unsupported Media Type",
            HttpStatusCode::RangeNotSatisfiable416 => "HTTP/1.1 416 Range Not Satisfiable",
            HttpStatusCode::ExpectationFailed417 => "HTTP/1.1 417 Expectation Failed",
            HttpStatusCode::ImATeapot418 => "HTTP/1.1 418 I'm a teapot",
            HttpStatusCode::MisdirectedRequest421 => "HTTP/1.1 421 Misdirected Request",
            HttpStatusCode::UnprocessableEntity422 => "HTTP/1.1 422 Unprocessable Entity",
            HttpStatusCode::Locked423 => "HTTP/1.1 423 Locked",
            HttpStatusCode::FailedDependency424 => "HTTP/1.1 424 Failed Dependency",
            HttpStatusCode::TooEarly425 => "HTTP/1.1 425 Too Early",
            HttpStatusCode::UpgradeRequired426 => "HTTP/1.1 426 Upgrade Required",
            HttpStatusCode::PreconditionRequired428 => "HTTP/1.1 428 Precondition Required",
            HttpStatusCode::TooManyRequests429 => "HTTP/1.1 429 Too Many Requests",
            HttpStatusCode::RequestHeaderFieldsTooLarge431 => "HTTP/1.1 431 Request Header Fields Too Large",
            HttpStatusCode::UnavailableForLegalReasons451 => "HTTP/1.1 451 Unavailable For Legal Reasons",
            HttpStatusCode::InternalServerError500 => "HTTP/1.1 500 Internal Server Error",
            HttpStatusCode::NotImplemented501 => "HTTP/1.1 501 Not Implemented",
            HttpStatusCode::BadGateway502 => "HTTP/1.1 502 Bad Gateway",
            HttpStatusCode::ServiceUnavailable503 => "HTTP/1.1 503 Service Unavailable",
            HttpStatusCode::GatewayTimeout504 => "HTTP/1.1 504 Gateway Timeout",
            HttpStatusCode::HttpVersionNotSupported505 => "HTTP/1.1 505 HTTP Version Not Supported",
            HttpStatusCode::VariantAlsoNegotiates506 => "HTTP/1.1 506 Variant Also Negotiates",
            HttpStatusCode::InsufficientStorage507 => "HTTP/1.1 507 Insufficient Storage",
            HttpStatusCode::LoopDetected508 => "HTTP/1.1 508 Loop Detected",
            HttpStatusCode::NotExtended510 => "HTTP/1.1 510 Not Extended",
            HttpStatusCode::NetworkAuthRequired511 => "HTTP/1.1 511 Network Authentication Required",};
        
        let headers = match self.headers.as_str(){
            Some(headers) => headers,
            None => "Content-Type: application/json"
        
        };
        let body = match self.body {
            Some(body) => body,
            None => "".into() 
        };
        let body: &[u8]  = &body;
        let new_line = b"\n".as_slice();
        response.push(status_code.as_bytes());
        response.push(new_line);
        response.push(headers.as_bytes());
        response.push(new_line);
        response.push(new_line);
        response.push(body);

        response.concat()
    }
}