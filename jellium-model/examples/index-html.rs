//! Writes `appearance::document::index()` to standard output, which is what
//! `just static-page` redirects into `jellium-web/index.html`.

fn main() {
    print!("{}", jellium_model::appearance::document::index());
}
