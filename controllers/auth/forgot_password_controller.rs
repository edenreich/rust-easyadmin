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
    // @todos
    // - check if the user exists by given email
    // - if exists, send that user an email to reset their password
    // - redirect to the same page with a flash message
    Redirect::to("/auth/forgot-password")
}