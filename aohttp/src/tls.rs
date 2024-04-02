use crate::logger::log;

struct ClientHello {
    protocol_version: u16,
    random: u16,
    session_id: str,
    cipher_suites: Vec<String>,
    compression_methods: u8,
    extensions: Vec<String>,
}

struct ServerHello {
    server_version: String,
    random: String,
    session_id: String,
    cipher_suite: String,
    compression_method: String,
    extensions: Vec<String>,
}

struct ServerHelloRetryRequest {
    server_version: String,
    random: u32,
    session_id: String,
    cipher_suite: String,
    compression_method: u8,
    extensions: Vec<String>,
}

struct ServerCertificate {
    certificate: String,
}

struct ServerKeyExchange {
    key_exchange: String,
}

struct ServerHelloDone {
    done: String,
}

pub fn parse_client_hello() {
    log("Parsing client hello", "logs/tls.log").unwrap();
}

pub fn server_hello() {
    log("Sending server hello", "logs/tls.log").unwrap();
}

pub fn server_certificate() {
    log("Sending server certificate", "logs/tls.log").unwrap();
}

pub fn server_key_exchange() {
    log("Sending server key exchange", "logs/tls.log").unwrap();
}   

pub fn server_hello_done() {
    log("Sending server hello done", "logs/tls.log").unwrap();
}

pub fn parse_client_key_exchange() {
    log("Receiving client key exchange", "logs/tls.log").unwrap();
}

pub fn parse_change_cipher_spec() {
    log("Receiving change cipher spec", "logs/tls.log").unwrap();
}   

pub fn parse_client_finished() {
    log("Receiving client finished", "logs/tls.log").unwrap();
}

pub fn server_change_cipher_spec() {
    log("Sending change cipher spec", "logs/tls.log").unwrap();
}   

pub fn server_finished() {
    log("Sending server finished", "logs/tls.log").unwrap();
}   

pub fn tls_handshake() {
    parse_client_hello();
    server_hello();
    server_certificate();
    server_key_exchange();
    server_hello_done();
    parse_client_key_exchange();
    parse_change_cipher_spec();
    parse_client_finished();
    server_change_cipher_spec();
    server_finished();
}   

