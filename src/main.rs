#[macro_use]
extern crate rocket;

mod nav;
mod views;

use nav::Page;

use propel_rocket::PropelView;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::Redirect;
use views::*;

#[get("/")]
fn home() -> Redirect {
    Redirect::to("/docs/introduction")
}

/// The documentation pages. The address must be in the sidebar, so only the
/// pages the navigation lists are served.
#[get("/docs/<page>")]
fn docs(page: &str) -> Result<PropelView, (Status, PropelView)> {
    let path = format!("/docs/{page}");

    match nav::title(&path) {
        Some(title) => Ok(PropelView::render(
            &format!("docs.{page}"),
            &Page::new(&path, Article { title }),
        )),
        None => Err(not_found_page()),
    }
}

fn not_found_page() -> (Status, PropelView) {
    (
        Status::NotFound,
        PropelView::render("errors.not_found", &Page::new("", NoFields {})),
    )
}

#[catch(404)]
fn not_found() -> (Status, PropelView) {
    not_found_page()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home, docs])
        // Built assets: hashed files and copied images and fonts.
        .mount("/build", FileServer::new("public/build"))
        .register("/", catchers![not_found])
}
