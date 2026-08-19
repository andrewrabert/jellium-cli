//! The static page this workspace serves, checked against the module that
//! renders it.

use jellium_model::appearance::document;

#[test]
fn the_served_document_is_the_one_the_appearance_module_renders() {
    assert_eq!(
        include_str!("../../jellium-web/index.html"),
        document::index(),
        "jellium-web/index.html has drifted from appearance::document::index; \
         `just static-page` writes it"
    );
}
