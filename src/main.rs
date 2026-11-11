#[macro_use]
extern crate rocket;

mod nav;
mod views;

use nav::DocPage;

use propel_rocket::PropelView;
use rocket::fs::{FileServer, Options};
use rocket::http::{ContentType, Status};
use rocket::response::Redirect;

/// The site's absolute address. Canonical links, the sitemap and the social
/// card are built from it.
pub const SITE_URL: &str = "https://ignite.rocketboost.rs";

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

/// The crawler rules. Everything is allowed, and the sitemap is named.
#[get("/robots.txt")]
fn robots() -> (ContentType, String) {
    (
        ContentType::Plain,
        format!("User-agent: *\nAllow: /\n\nSitemap: {SITE_URL}/sitemap.xml\n"),
    )
}

/// Every documentation page in the sidebar, in order.
#[get("/sitemap.xml")]
fn sitemap() -> (ContentType, String) {
    let urls: String = nav::paths()
        .iter()
        .map(|path| {
            format!(
                "  <url>\n    <loc>{SITE_URL}{path}</loc>\n    <lastmod>2026-11-09</lastmod>\n  </url>\n"
            )
        })
        .collect();
    (
        ContentType::new("application", "xml"),
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{urls}</urlset>\n"
        ),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home, docs, robots, sitemap])
        // Built assets: hashed files and copied images and fonts.
        .mount(
            "/build",
            // Same as the 0.6-dev `FileServer::new`: index files and trailing slashes for directories.
            FileServer::new("public/build", Options::Index | Options::NormalizeDirs),
        )
        .register("/", catchers![not_found])
}
