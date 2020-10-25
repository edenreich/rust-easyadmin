use easyadmin::thirdparty::rocket::{self, get, post};
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

#[post("/register", format = "text/html")]
pub fn submit() -> Option<AuthView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("version", "0.1.0");

    // @todo check the user's credentials, if it's valid redirect to dashboard,
    // otherwise stay on the same page and display errors.
    let mut view: AuthView = AuthView::new("register");
    view.set(data);
    Some(view)
}
