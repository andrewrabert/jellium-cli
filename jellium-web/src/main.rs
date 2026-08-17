#![deny(unused_must_use)]
#![deny(clippy::let_underscore_must_use)]
#![deny(let_underscore_drop)]
#![deny(clippy::disallowed_methods)]

mod api;
mod app;
mod boot;
mod browser;
mod control;
mod error;
mod failure;
mod fonts;
mod identity;
mod images;
mod live;
mod livetv;
mod overlay;
mod player;
mod prefs;
mod profile;
#[cfg(test)]
mod reference;
mod route;
mod screen;
mod settings;
mod style;
mod text;
mod theme;
mod viewport;
mod widget;
mod window;

fn main() {
    crate::failure::install_panic_hook();
    console_log::init_with_level(log::Level::Warn).expect("the console logger is installed once");
    wasm_bindgen_futures::spawn_local(crate::boot::start());
}
