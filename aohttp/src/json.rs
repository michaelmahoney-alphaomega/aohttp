use std::{
    net::TcpStream,
    io::BufReader,
};

use serde_json::Value;

pub fn request_to_json(mut tcp_stream: TcpStream) -> Value {
    let buf_reader=  BufReader::new(&mut tcp_stream);
    let request: Value = serde_json::from_reader(buf_reader).unwrap();
    // let buf =  buf_reader.buffer();
    // if buf.is_empty() 
    //     {let buf_temp: &[u8] = &[0][..];} 
    // let request: Value = serde_json::from_slice(buf).unwrap();
    return request
}

pub fn json_to_response() {
    
}