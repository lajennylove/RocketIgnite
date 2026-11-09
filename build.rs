//! Compiles the documentation views into Rust, so a release build renders them
//! without a template interpreter. The generated module is included in `views.rs`.

fn main() {
    const PAGE: &str = "crate::nav::DocPage";
    propel_compiler::build::Views::new("resources/views")
        .view("docs.introduction", PAGE)
        .view("docs.install", PAGE)
        .view("docs.panel", PAGE)
        .view("docs.resources", PAGE)
        .view("docs.forms", PAGE)
        .view("docs.tables", PAGE)
        .view("docs.actions", PAGE)
        .view("docs.notifications", PAGE)
        .view("docs.widgets", PAGE)
        .view("docs.views", PAGE)
        .view("errors.not_found", PAGE)
        .build();
}
