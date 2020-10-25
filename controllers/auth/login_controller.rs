use easyadmin::thirdparty::rocket::{self, response::Redirect, get, post};
use easyadmin::view::AuthView;
use std::collections::HashMap;

#[get("/login", format = "text/html")]
pub fn form() -> Option<AuthView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("version", "0.1.0");
    let mut view: AuthView = AuthView::new("login");
    view.set(data);
    Some(view)
}

#[post("/login")]
pub fn submit() -> Redirect {
    // @todos
    // - check the user credentials
    // - if exists / valid, send session cookie with the client session id
    // - if not exists / not valid, send error validation to the client 
    // - redirect the user to the dashboard
    Redirect::to("/auth/login")
}
