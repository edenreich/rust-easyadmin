use easyadmin::view::FrontendView;
use std::collections::HashMap;

#[get("/", format = "text/html")]
pub fn index() -> Option<FrontendView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("version", "0.1.0");
    // data.insert("logged_in", "true");
    let mut view: FrontendView = FrontendView::new("index");
    view.set(data);
    Some(view)
}
