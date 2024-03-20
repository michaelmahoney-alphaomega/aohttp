use aohttp::*;

fn main() 
{
    println!("Hello, world!");
    // let http_server = create_http_server("127.0.0.1:8080").unwrap();
    let routes = true;
    let http_server = run_http_server("127.0.0.1", routes)
    
}
