//! The page's own size, read from the window that reports it.

use crate::failure::{self, Call};
use crate::style::{Css, Viewport};

/// The page's own size, from `window.innerWidth` and `window.innerHeight`,
/// which report css pixels; `None` where the page reports none.
pub fn read() -> Option<Viewport> {
    let window = web_sys::window()?;
    let width = failure::called(Call::WindowInnerWidth, window.inner_width())?.as_f64()?;
    let height = failure::called(Call::WindowInnerHeight, window.inner_height())?.as_f64()?;
    Some(Viewport::new(Css::of(width as f32), Css::of(height as f32)))
}
