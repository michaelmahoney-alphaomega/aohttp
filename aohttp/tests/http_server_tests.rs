use core::panic;

use aohttp;
use aohttp::http::*;
use aohttp::router::*;

fn test_constant_setup()
{
    let auth: HttpAuth = HttpAuth::Basic(String::from("this is a thing"));
    const uri: Uri = Uri {
    path: String::from("test_path"), 
    query: None, 
    fragment: None 
};
const handler: Option<fn(&Route) -> Result<HttpResponse, Error>> = None;
const test_route: Route = Route{
    auth: auth,
    uri: uri,
    handler: handler
};
}




#[test]
fn server_ping_test()
{
    const routes: Vec<&Route> = vec![&test_route];
    let result = aohttp::run_http_server("127.0.0.1", &routes);
    match result 
    {
        Ok(_) => (),
        Err(_) => panic!{"It didn't work."}
    }
    
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