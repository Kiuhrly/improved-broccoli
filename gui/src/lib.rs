#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

mod screen_ui;
mod keyboard;
mod settings;

mod app;
pub use app::App;

use rust_i18n::i18n;
i18n!("locales");

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;