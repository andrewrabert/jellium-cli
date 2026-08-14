use crate::text::{self, Text};

fn boot_element() -> Option<web_sys::Element> {
    web_sys::window()?.document()?.get_element_by_id("jellium-boot")
}

pub fn hide_static_page() {
    if let Some(element) = boot_element() {
        let _ = element.set_attribute("hidden", "");
    }
}

pub fn report_failure(key: Text) {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    if let Some(message) = document.get_element_by_id("jellium-boot-message") {
        message.set_text_content(Some(text::lookup(key)));
    }
    if let Some(element) = document.get_element_by_id("jellium-boot") {
        let _ = element.remove_attribute("hidden");
        let _ = element.set_attribute("data-state", "failed");
    }
}

pub fn install_panic_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        report_failure(Text::BootRendererFailed);
        previous(info);
    }));
}
