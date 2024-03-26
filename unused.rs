// fn collect_stream(tcp_stream_ref: &TcpStream) -> Value {
//     let buf_reader =  BufReader::new(tcp_stream_ref);
//     let request: Vec<String> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line: &String| !line.is_empty())
//         .collect();

//     let request: Value = json!(request); //convert Vec to serde_json::Value
//     return request}