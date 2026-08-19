//! Writes `appearance::css::boot()` to standard output, which is what
//! `just static-page` redirects into `jellium-web/boot.css`.

fn main() {
    print!("{}", jellium_model::appearance::css::boot());
}
