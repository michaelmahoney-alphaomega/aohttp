use core::panic;
use aohttp;
use aohttp::http::*;
use aohttp::router::*;

fn test_constant_setup() -> Route {
    let auth: HttpAuth = HttpAuth::Basic(String::from("this is a thing"));

    let uri: Uri = Uri {
        path: String::from("/test_path"), 
        query: None, 
        fragment: None };

    let handler: Option<fn(&Route) -> Result<HttpResponse, Error>> = None;

    let test_route: Route = Route {
        auth: auth,
        uri: uri,
        handler: handler
    };

    return test_route;
} 




#[test]
fn server_standup_test(){
    let test_route = test_constant_setup();
    let routes: Vec<&Route> = vec![&test_route];
    let result = aohttp::run_http_server("127.0.0.1:8080", &routes);
    match result {
        Ok(_) => (),
        Err(error) => panic!{"It didn't work. Here's an error {:?}", error}}}

#[test]
fn log_test() {
    let message = "INFO: this is a test";
    let answer = aohttp::logger::log(&mut String::from(message), "logs/tests.log");
    match answer {
        Ok(_answer) => println!("This is the ok {}", message),
        Err(answer) => panic!("This is the error {}", answer)}}