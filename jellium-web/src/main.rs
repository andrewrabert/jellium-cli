#![deny(unused_must_use)]
#![deny(clippy::let_underscore_must_use)]
#![deny(let_underscore_drop)]
#![deny(clippy::disallowed_methods)]

mod api;
mod app;
mod boot;
mod control;
mod error;
mod failure;
mod images;
mod live;
mod livetv;
mod overlay;
mod player;
mod prefs;
mod route;
mod screen;
mod text;
mod theme;
mod widget;
mod window;

fn main() {
    crate::failure::install_panic_hook();
    console_log::init_with_level(log::Level::Warn).expect("the console logger is installed once");
    wasm_bindgen_futures::spawn_local(crate::boot::start());
}
