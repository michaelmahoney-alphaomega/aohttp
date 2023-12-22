use aohttp;

#[test]
fn server_ping_test()
{

}

#[test]
fn log_test()
{
    let answer = aohttp::logger::log(&mut "this is a test", "http_server.log");
    match answer {
        Ok(_answer) => (),
        Err(_answer) => panic!()
    }
}