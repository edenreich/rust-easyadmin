use easyadmin::thirdparty::rocket::{self, response::Redirect, get, post};
use easyadmin::view::AuthView;
use std::collections::HashMap;

#[get("/register", format = "text/html")]
pub fn form() -> Option<AuthView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("version", "0.1.0");
    let mut view: AuthView = AuthView::new("register");
    view.set(data);
    Some(view)
}

#[post("/register")]
pub fn submit() -> Redirect {
    // @todos
    // - create a new account and persist to the database
    // - send session cookie with the client session id
    // - redirect the user to the dashboard
    Redirect::to("/admin/dashboard")
}

