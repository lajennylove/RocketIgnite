//! The site's navigation: the sidebar groups, and the previous and next page
//! for each page. Both come from one ordered list, so the sidebar and the pager
//! always agree.

use serde::Serialize;

/// A link is true when it exists, so `@if ($pager.prev)` reads an `Option<Link>`.
#[derive(Debug, Clone, Serialize, propel_macros::Truthy)]
pub struct Link {
    pub href: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct Group {
    pub heading: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize)]
pub struct Pager {
    pub prev: Option<Link>,
    pub next: Option<Link>,
}

/// Every page in the sidebar, in order. Previous and next follow this order.
/// The groups are the sections of the site: getting started, the admin panel,
/// the building blocks of a resource, and views.
fn groups() -> Vec<Group> {
    let group = |heading: &str, links: &[(&str, &str)]| Group {
        heading: heading.into(),
        links: links
            .iter()
            .map(|(href, label)| Link {
                href: (*href).into(),
                label: (*label).into(),
            })
            .collect(),
    };

    vec![
        group(
            "Getting started",
            &[
                ("/docs/introduction", "Introduction"),
                ("/docs/install", "Installation"),
            ],
        ),
        group("Admin panel", &[("/docs/panel", "Panel")]),
        group(
            "Resources",
            &[
                ("/docs/resources", "Resources"),
                ("/docs/forms", "Forms"),
                ("/docs/tables", "Tables"),
                ("/docs/actions", "Actions"),
                ("/docs/notifications", "Notifications"),
                ("/docs/widgets", "Widgets"),
            ],
        ),
        group("Customizing", &[("/docs/views", "Views")]),
    ]
}

/// The sidebar title of the page at `path`, if the page is in the sidebar.
pub fn title(path: &str) -> Option<String> {
    groups()
        .into_iter()
        .flat_map(|group| group.links)
        .find(|link| link.href == path)
        .map(|link| link.label)
}

/// The context of every documentation page. The layout reads `title`, `nav` and
/// `pager` as top-level fields, because a compiled view reads each field as
/// `ctx.field`.
#[derive(Debug, Serialize)]
pub struct DocPage {
    pub title: String,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

impl DocPage {
    /// `path` is the page's address. Only pages in the sidebar get a previous
    /// and a next link.
    pub fn new(path: &str, title: &str) -> Self {
        let nav = groups();
        let links: Vec<Link> = nav.iter().flat_map(|group| group.links.clone()).collect();
        let index = links.iter().position(|link| link.href == path);

        let pager = Pager {
            prev: index
                .and_then(|i| i.checked_sub(1))
                .map(|i| links[i].clone()),
            next: index.and_then(|i| links.get(i + 1).cloned()),
        };

        DocPage {
            title: title.into(),
            nav,
            pager,
        }
    }
}
