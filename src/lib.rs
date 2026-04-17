#![warn(clippy::all, rust_2018_idioms)]

mod manga_data;
mod review_entry;
mod rating_entry;
mod ui;
mod app;

pub use app::TemplateApp;
pub use rating_entry::RatingEntry;
pub use review_entry::ReviewEntry;
pub use manga_data::MangaData;