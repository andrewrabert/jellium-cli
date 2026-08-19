pub struct Asset {
    pub path: &'static str,
    pub content_type: &'static str,
    pub bytes: &'static [u8],
}

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

pub fn lookup(path: &str) -> Option<&'static Asset> {
    let path = match path.trim_start_matches('/') {
        "" => "index.html",
        path => path,
    };
    ASSETS.iter().find(|asset| asset.path == path)
}
