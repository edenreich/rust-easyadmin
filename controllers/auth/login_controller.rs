extern crate easyadmin;
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

#[post("/login", format = "text/html")]
pub fn submit() -> Option<AuthView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("version", "0.1.0");

    // @todo check the user's credentials, if it's valid redirect to dashboard,
    // otherwise stay on the same page and display errors.
    let mut view: AuthView = AuthView::new("login");
    view.set(data);
    Some(view)
}
