#[macro_use]
extern crate rocket;

mod nav;
mod views;

use nav::DocPage;

use propel_rocket::PropelView;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::Redirect;

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
        Some(title) => Ok(views::render_page(
            &format!("docs.{page}"),
            &DocPage::new(&path, &title),
        )),
        None => Err(not_found_page()),
    }
}

fn not_found_page() -> (Status, PropelView) {
    (
        Status::NotFound,
        views::render_page("errors.not_found", &DocPage::new("", "Not found")),
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
