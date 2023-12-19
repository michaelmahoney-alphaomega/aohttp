extern crate serde_json; 

use std::{
    net::TcpStream,
    io::{prelude::*, BufReader, BufWriter}};

use serde_json::{Value, json};

use crate::auth::*;
use crate::users::*;
use crate::greenhouse::*;
use crate::games::*;
use crate::dates::*;

use crate::util::http::*;

use super::http;

fn auth_router(http_request: &HttpRequest) -> HttpResponse {
    let auth_response = match &http_request.auth {
        HttpAuth::Basic(_) => handle_basic_auth(http_request),
        HttpAuth::Modern(_) => handle_modern_auth(http_request),
        HttpAuth::OAuth(_) => handle_oauth_auth(http_request),
        HttpAuth::OAuth2(_) => handle_oauth2_auth(http_request)};
        return auth_response}

fn dates_router(http_request: &HttpRequest) -> HttpResponse {
    let dates_response = handle_dates(http_request);
    return dates_response}

fn games_router(http_request: &HttpRequest) -> HttpResponse {
    let games_response = handle_games(http_request);
    return games_response}

fn users_router(http_request: &HttpRequest) -> HttpResponse {
    let users_response = handle_users(http_request);
    return users_response}

fn greenhouse_router(http_request: &HttpRequest) -> HttpResponse {
    let users_response = handle_greenhouse(http_request);
    return users_response}

fn collect_stream(tcp_stream_ref: &TcpStream) -> Value {
    let buf_reader =  BufReader::new(tcp_stream_ref);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line: &String| !line.is_empty())
        .collect();

    let request: Value = json!(request); //convert Vec to serde_json::Value
    return request}

pub fn main_router(tcp_stream: TcpStream) -> () {
    let http_request = match HttpRequest::build_from_stream(&tcp_stream) {
        Ok(http_request) => http_request,
        Err(e) => panic!("This is broken, here's the error: {e}")};

    let response = match &http_request.uri {
            ApiResource::Auth(_) => auth_router(&http_request),
            ApiResource::Users(_) => users_router(&http_request),
            ApiResource::Games(_) => dates_router(&http_request),
            ApiResource::Dates(_) => games_router(&http_request),
            ApiResource::Greenhouse(_) => greenhouse_router(&http_request)};

    let mut buf_writer = BufWriter::new(tcp_stream);
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