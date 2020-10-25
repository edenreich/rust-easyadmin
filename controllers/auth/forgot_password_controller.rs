use easyadmin::thirdparty::rocket::{self, response::Redirect, get, post};
use easyadmin::view::AuthView;
use std::collections::HashMap;

#[get("/forgot-password", format = "text/html")]
pub fn form() -> Option<AuthView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("version", "0.1.0");
    let mut view: AuthView = AuthView::new("forgot_password");
    view.set(data);
    Some(view)
}

#[post("/forgot-password")]
pub fn submit() -> Redirect {
    // @todo check the user's credentials, if it's valid redirect to dashboard,
    // otherwise stay on the same page and display errors.
    Redirect::to("/auth/forgot-password")
}