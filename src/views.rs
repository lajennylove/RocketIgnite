//! Typed contexts for the views in `resources/views/`.
//!
//! Each view has one context struct here, and handlers build it. Today a
//! template that names a field the context lacks fails at render time. Once
//! views compile, the same mistake fails at `cargo check`.

use serde::Serialize;

/// A page with only a title, such as the documentation pages.
#[derive(Debug, Serialize)]
pub struct Article {
    pub title: String,
}

/// A context with no fields, for views that show a fixed message.
#[derive(Debug, Serialize)]
pub struct NoFields {}
