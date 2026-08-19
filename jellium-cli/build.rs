use std::path::{Path, PathBuf};

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        Some("ttf") => "font/ttf",
        Some("png") => "image/png",
        Some("ico") => "image/vnd.microsoft.icon",
        Some("woff2") => "font/woff2",
        Some("tsv") => "text/tab-separated-values; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(root, &path, out);
        } else {
            let relative = path
                .strip_prefix(root)
                .expect("asset is inside the dist directory")
                .to_string_lossy()
                .replace('\\', "/");
            out.push((relative, path));
        }
    }
}

fn build_bundle(out_dir: &Path) -> PathBuf {
    let dist = out_dir.join("jellium-web-dist");
    let status = std::process::Command::new("trunk")
        .args(["build", "--release", "--dist"])
        .arg(&dist)
        .arg("../jellium-web/index.html")
        .status()
        .expect("could not run trunk; install it with `cargo install --locked trunk`");
    assert!(
        status.success(),
        "trunk failed to build the Jellium Web bundle"
    );
    dist
}

fn main() {
    println!("cargo::rerun-if-env-changed=JELLIUM_WEB_DIST");
    println!("cargo::rerun-if-changed=../jellium-web/src");
    println!("cargo::rerun-if-changed=../jellium-web/strings");
    println!("cargo::rerun-if-changed=../jellium-web/js");
    println!("cargo::rerun-if-changed=../jellium-web/vendor");
    println!("cargo::rerun-if-changed=../jellium-web/branding");
    println!("cargo::rerun-if-changed=../jellium-web/fonts");
    println!("cargo::rerun-if-changed=../jellium-web/icons");
    println!("cargo::rerun-if-changed=../jellium-web/index.html");
    println!("cargo::rerun-if-changed=../jellium-web/boot.js");
    println!("cargo::rerun-if-changed=../jellium-web/boot.css");
    println!("cargo::rerun-if-changed=../jellium-web/Cargo.toml");
    println!("cargo::rerun-if-changed=../jellium-web/Trunk.toml");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR is set by cargo"));

    let dist = match std::env::var_os("JELLIUM_WEB_DIST") {
        Some(prebuilt) => PathBuf::from(prebuilt),
        None => build_bundle(&out_dir),
    };

    let mut assets = Vec::new();
    collect(&dist, &dist, &mut assets);
    assets.sort();

    let mut source = String::from("pub static ASSETS: &[Asset] = &[\n");
    for (relative, path) in &assets {
        source.push_str(&format!(
            "    Asset {{ path: {:?}, content_type: {:?}, bytes: include_bytes!({:?}) }},\n",
            relative,
            content_type(path),
            path,
        ));
    }
    source.push_str("];\n");

    std::fs::write(out_dir.join("assets.rs"), source).expect("could not write assets.rs");
}
