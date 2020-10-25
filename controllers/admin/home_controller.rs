use easyadmin::thirdparty::rocket::{self, response::Redirect, get};
use easyadmin::view::AdminView;
use std::collections::HashMap;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/admin/dashboard")
}

#[get("/", format = "text/html")]
pub fn dashboard() -> Option<AdminView> {
    // @todos
    // - add authentication guard
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("version", "0.1.0");
    let mut view: AdminView = AdminView::new("dashboard");
    view.set(data);
    Some(view)
}
