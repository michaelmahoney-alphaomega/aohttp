use aohttp;

#[test]
fn server_ping_test()
{

}

#[test]
fn log_test()
{
    let message = "INFO: this is a test";
    let answer = aohttp::logger::log(&mut String::from(message), "logs/tests.log");
    match answer {
        Ok(_answer) => println!("This is the ok {}", message),
        Err(answer) => panic!("This is the error {}", answer)
    }
}