mod api;
mod app;
mod boot;
mod control;
mod error;
mod images;
mod route;
mod screen;
mod text;
mod theme;
mod widget;

fn main() -> iced::Result {
    boot::install_panic_hook();

    iced::application(app::Jellium::boot, app::Jellium::update, app::Jellium::view)
        .title(app::Jellium::title)
        .theme(app::Jellium::theme)
        .run()
}
