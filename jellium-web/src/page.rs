//! What the page reports about itself, read from the window that reports it.

use crate::failure::{self, Call};
use crate::style::{Css, Screen, Viewport};

/// The page's own size, from `window.innerWidth` and `window.innerHeight`,
/// which report css pixels; `None` where the page reports none.
pub fn viewport() -> Option<Viewport> {
    let window = web_sys::window()?;
    let width = failure::called(Call::WindowInnerWidth, window.inner_width())?.as_f64()?;
    let height = failure::called(Call::WindowInnerHeight, window.inner_height())?.as_f64()?;
    Some(Viewport::new(Css::of(width as f32), Css::of(height as f32)))
}

/// What the display offers, from `screen.availWidth`, which is what decides
/// whether an image request rounds the page width down to a hundred; `None`
/// where the page reports none.
pub fn screen() -> Option<Screen> {
    let window = web_sys::window()?;
    let screen = failure::called(Call::WindowScreen, window.screen())?;
    let available = failure::called(Call::ScreenAvailWidth, screen.avail_width())?;
    Some(Screen::new(Css::of(available as f32)))
}

/// The origin every same-origin url is built on, and the empty string where the
/// page reports none, which leaves those urls relative and so still same-origin.
pub fn origin() -> String {
    let Some(window) = web_sys::window() else {
        return String::new();
    };
    failure::called(Call::LocationOrigin, window.location().origin()).unwrap_or_default()
}
