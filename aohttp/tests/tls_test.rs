use aohttp;
use aohttp::tls::*;
use aohttp::router::*;
use rand::Rng;

fn test_constant_setup() -> 
(
    ClientHello,//, 
    ServerHello, 
    // ServerHelloRetryRequest, 
    // ServerCertificate, 
    // ServerKeyExchange, 
    // ServerHelloDone, 
    // ClientKeyExchange, 
    // ChangeCipherSpec, 
    // ClientFinished, 
    // ServerChangeCipherSpec, 
    // ServerFinished
) 
{
    let mut rng = rand::thread_rng();
    let client_hello: ClientHello = ClientHello {
        protocol_version: 0x0303 as u16,
        random: rng.gen(),
        cipher_suites: (0x13, 0x01),
        legacy_compression_methods: 0x00 as u8,
        extensions: Vec::new(),
        legacy_session_id:  false as u32,
    };

    let server_hello: ServerHello = ServerHello {
        protocol_version: 0x0303 as u16,
        random: rng.gen(),
        session_id: 0x0000 as u32,
        cipher_suite: (0x13, 0x01),
        compression_method: 0,
        extensions: Vec::new(),
    };



    (client_hello, server_hello)
} 

#[test]
fn test_client_hello() 
{
    let handshake = test_constant_setup();
    let result = parse_client_hello();
    match result 
    {
        Ok(_) => (),
        Err(error) => panic!{"It didn't work. Here's an error {:?}", error}
    }

}