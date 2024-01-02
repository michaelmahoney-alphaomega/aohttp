// Import your handle functions into this module and create a funtion that returns the ApiResource 
// if you want you can just define them in the router mod, but it might get too crowded.
// The auth_handler function is a dummy example of what to do.

pub struct ApiResource {
    path: &str,
    handler: fn(&Route) -> Result<HttpResponse, Error>
}

pub fn auth_handler() -> () {}

pub fn return_auth() -> ApiResource {
    let auth = ApiResource::new();
    auth.path = "auth/";
    auth.handler = auth_handler;
    return auth
}
// 