// fn collect_stream(tcp_stream_ref: &TcpStream) -> Value {
//     let buf_reader =  BufReader::new(tcp_stream_ref);
//     let request: Vec<String> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line: &String| !line.is_empty())
//         .collect();

//     let request: Value = json!(request); //convert Vec to serde_json::Value
//     return request}

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