use crate::logger::log;
use rand::Rng;

pub struct ClientHello {
    pub protocol_version: u16,
    pub random: u32,
    pub legacy_session_id: u32,
    pub cipher_suites: (u32,u16),
    pub legacy_compression_methods: u8,
    pub extensions: Vec<u16>,
}

pub struct ServerHello {
    pub protocol_version: u16,
    pub random: u32,
    pub session_id: u32,
    pub cipher_suite: (u16,u16),
    pub compression_method: u8,
    pub extensions: Vec<u16>,
}

struct ServerHelloRetryRequest {
    pub protocol_version: u16,
    pub random: u32,
    pub session_id: u32,
    pub cipher_suite: (u8,u8),
    pub compression_method: u8,
    pub extensions: Vec<u16>,

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

struct ClientKeyExchange {
    key_exchange: String,
}

struct ChangeCipherSpec {
    spec: String,
}

struct ClientFinished {
    finished: String,
}

struct ServerChangeCipherSpec {
    spec: String,
}

struct ServerFinished {
    finished: String,
}   

struct TslExtension {
    extension_type: TslExtensionType,
    extension_data: u16

}

pub enum TslExtensionType {
    ServerName,
    MaxFragmentLength,
    StatusRequest,
    SupportedGroups,
    SignatureAlgorithms,
    UseSRTP,
    Heartbeat,
    ApplicationLayerProtocolNegotiation,
    SignedCertificateTimestamp,
    ClientCertificateType,
    ServerCertificateType,
    Padding,
    PreSharedKey,
    EarlyData,
    SupportedVersions,
    Cookie,
    PSKKeyExchangeModes,
    CertificateAuthorities,
    OIDFilters,
    PostHandshakeAuth,
    SignatureAlgorithmsCert,
    KeyShare,
}

// pub fn parse_client_hello() {
//     log("Parsing client hello", "logs/tls.log").unwrap();
// }

pub fn server_hello() {
    let server_hello = ServerHello {
        protocol_version: 0x0303,
        random: rand::thread_rng().gen(),
        session_id: rand::thread_rng().gen(),
        cipher_suite: (0x1301, 0x1302),
        compression_method: 0,
        extensions: vec![0x0000],
    };
    let mut message = String::from("Sending server hello");
    log(&mut message, "logs/tls.log").unwrap();
}

// pub fn server_certificate() {
//     log("Sending server certificate", "logs/tls.log").unwrap();
// }

// pub fn server_key_exchange() {
//     log("Sending server key exchange", "logs/tls.log").unwrap();
// }   

// pub fn server_hello_done() {
//     log("Sending server hello done", "logs/tls.log").unwrap();
// }

// pub fn parse_client_key_exchange() {
//     log("Receiving client key exchange", "logs/tls.log").unwrap();
// }

// pub fn parse_change_cipher_spec() {
//     log("Receiving change cipher spec", "logs/tls.log").unwrap();
// }   

// pub fn parse_client_finished() {
//     log("Receiving client finished", "logs/tls.log").unwrap();
// }

// pub fn server_change_cipher_spec() {
//     log("Sending change cipher spec", "logs/tls.log").unwrap();
// }   

// pub fn server_finished() {
//     log("Sending server finished", "logs/tls.log").unwrap();
// }   

// pub fn tls_handshake() {
//     parse_client_hello();
//     server_hello();
//     server_certificate();
//     server_key_exchange();
//     server_hello_done();
//     parse_client_key_exchange();
//     parse_change_cipher_spec();
//     parse_client_finished();
//     server_change_cipher_spec();
//     server_finished();
// }   

