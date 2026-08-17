//! Writes `appearance::css::boot()` to standard output, which is what
//! `just boot-css` redirects into `jellium-web/boot.css`.

fn main() {
    print!("{}", jellium_model::appearance::css::boot());
}
