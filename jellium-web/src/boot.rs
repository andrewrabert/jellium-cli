use std::cell::{Cell, RefCell};

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

use crate::failure::Call;
use crate::failure::{self, Cause, Failure};
use crate::style::typeface;
use crate::text::{self, Text};

thread_local! {
    /// The retry control's one click handler, installed the first time the
    /// control is shown and kept for the life of the page.
    static RETRY: RefCell<Option<Closure<dyn FnMut()>>> = const { RefCell::new(None) };

    /// True while a `start` is in flight, which is what keeps a second click
    /// from running the application twice.
    static STARTING: Cell<bool> = const { Cell::new(false) };
}

/// The graphics API a backend runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Api {
    WebGpu,
    WebGl2,
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Api::WebGpu => "WebGPU",
            Api::WebGl2 => "WebGL2",
        })
    }
}

/// The graphics backend the pre-flight found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backend {
    pub api: Api,
    /// The adapter as wgpu names it.
    pub adapter: String,
    /// The driver as wgpu names it.
    pub driver: String,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on {} ({})", self.api, self.adapter, self.driver)
    }
}

fn boot_element() -> Option<web_sys::Element> {
    web_sys::window()?
        .document()?
        .get_element_by_id("jellium-boot")
}

pub fn hide_static_page() {
    if let Some(element) = boot_element() {
        failure::called(
            Call::ElementSetAttribute,
            element.set_attribute("hidden", ""),
        );
    }
}

/// Shows `sentence` on the boot page, with the retry control shown only when
/// `retry` is offered.
fn show(sentence: &str, retry: bool) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    if let Some(message) = document.get_element_by_id("jellium-boot-message") {
        message.set_text_content(Some(sentence));
    }
    if let Some(element) = document.get_element_by_id("jellium-boot") {
        failure::called(
            Call::ElementRemoveAttribute,
            element.remove_attribute("hidden"),
        );
        failure::called(
            Call::ElementSetAttribute,
            element.set_attribute("data-state", "failed"),
        );
    }
    let Some(button) = document.get_element_by_id("jellium-boot-retry") else {
        return;
    };
    if !retry {
        failure::called(
            Call::ElementSetAttribute,
            button.set_attribute("hidden", ""),
        );
        return;
    }
    RETRY.with(|held| {
        if held.borrow().is_some() {
            return;
        }
        let again = Closure::<dyn FnMut()>::new(|| {
            wasm_bindgen_futures::spawn_local(start());
        });
        failure::called(
            Call::AddEventListener,
            button
                .unchecked_ref::<web_sys::EventTarget>()
                .add_event_listener_with_callback("click", again.as_ref().unchecked_ref()),
        );
        *held.borrow_mut() = Some(again);
    });
    failure::called(
        Call::ElementRemoveAttribute,
        button.remove_attribute("hidden"),
    );
}

/// Renders `sentence` on the boot page and shows the retry that re-runs
/// `start` in this page.
/// The retry's click handler is installed once and kept for the life of the
/// page, so a second refusal adds no second handler.
pub fn refuse(sentence: &str) {
    show(sentence, true);
}

/// Renders `sentence` on the boot page with no retry, which is what a trapped
/// module gets.
pub fn stopped(sentence: &str) {
    show(sentence, false);
}

/// The string a WebGL2 context answers for `parameter`, or `unknown`.
fn gl_parameter(context: &wasm_bindgen::JsValue, parameter: u32) -> String {
    let Some(get) = failure::called(
        Call::ReflectGet,
        js_sys::Reflect::get(context, &"getParameter".into()),
    ) else {
        return "unknown".to_owned();
    };
    let Some(get) = get.dyn_ref::<js_sys::Function>() else {
        return "unknown".to_owned();
    };
    match failure::called(
        Call::WebGl2GetParameter,
        get.call1(context, &parameter.into()),
    ) {
        Some(value) => value.as_string().unwrap_or_else(|| "unknown".to_owned()),
        None => "unknown".to_owned(),
    }
}

/// A WebGL2 context on a throwaway canvas, and `None` where the browser offers
/// none.
fn webgl2_context() -> Option<wasm_bindgen::JsValue> {
    let document = web_sys::window()?.document()?;
    let canvas = failure::called(
        Call::DocumentCreateElement,
        document.create_element("canvas"),
    )?;
    let get = failure::called(
        Call::ReflectGet,
        js_sys::Reflect::get(canvas.as_ref(), &"getContext".into()),
    )?;
    let context = failure::called(
        Call::HtmlCanvasElementGetContext,
        get.dyn_ref::<js_sys::Function>()?
            .call1(canvas.as_ref(), &"webgl2".into()),
    )?;
    if context.is_null() || context.is_undefined() {
        return None;
    }
    Some(context)
}

/// Requests a WebGPU adapter through wgpu's own browser detection and probes
/// for a WebGL2 context, answering the backend iced will run on.
/// A browser offering neither answers wgpu's refusal as the machine cause.
pub async fn preflight() -> Result<Backend, String> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
    let refusal = match instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
    {
        Ok(adapter) => {
            let info = adapter.get_info();
            return Ok(Backend {
                api: Api::WebGpu,
                adapter: info.name,
                driver: format!("{} {}", info.driver, info.driver_info)
                    .trim()
                    .to_owned(),
            });
        }
        Err(refused) => refused.to_string(),
    };

    const RENDERER: u32 = 0x1F01;
    const VERSION: u32 = 0x1F02;
    match webgl2_context() {
        Some(context) => Ok(Backend {
            api: Api::WebGl2,
            adapter: gl_parameter(&context, RENDERER),
            driver: gl_parameter(&context, VERSION),
        }),
        None => Err(refusal),
    }
}

/// Pre-flights the backends, writes the one it found to the console and runs
/// the application on it; a browser offering neither renders the refusal on
/// the boot page and starts nothing.
/// A `start` already in flight answers at once, so no click and no retry ever
/// runs the application twice.
/// An `Err` from the run itself is raised as a failure.
pub async fn start() {
    if STARTING.with(|held| held.replace(true)) {
        return;
    }
    let backend = match preflight().await {
        Ok(backend) => backend,
        Err(refusal) => {
            STARTING.with(|held| held.set(false));
            let failure = Failure::told(Text::BootNoBackend, Cause::Graphics { detail: refusal });
            failure::record(&failure);
            refuse(&failure.sentence);
            return;
        }
    };
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "Jellium Web graphics backend: {backend}"
    )));

    let layout = crate::browser::Browser::detect(&crate::browser::Runtime::probe()).layout();
    let Some(viewport) = crate::page::viewport(layout) else {
        STARTING.with(|held| held.set(false));
        let failure = Failure::told(
            Text::BootNoViewport,
            Cause::Graphics {
                detail: backend.to_string(),
            },
        );
        failure::record(&failure);
        refuse(&failure.sentence);
        return;
    };

    let mut application = iced::application(
        move || crate::app::Jellium::boot(viewport),
        crate::app::Jellium::update,
        crate::app::Jellium::view,
    )
    .settings(iced::Settings {
        default_font: crate::style::font(typeface::Weight::Regular),
        default_text_size: iced::Pixels(crate::style::drawn(typeface::BODY.drawn())),
        ..iced::Settings::default()
    })
    .scale_factor(crate::app::Jellium::scale_factor);
    for face in crate::fonts::embedded() {
        application = application.font(face);
    }
    let run = application
        .title(crate::app::Jellium::title)
        .theme(crate::app::Jellium::theme)
        .style(crate::app::Jellium::style)
        .subscription(crate::app::Jellium::subscription)
        .transparent(true)
        .run();

    STARTING.with(|held| held.set(false));
    if let Err(error) = run {
        let failure = Failure::saying(
            text::format(Text::BootPanicked, &[&error.to_string()]),
            Cause::Graphics {
                detail: error.to_string(),
            },
        );
        failure::record(&failure);
        refuse(&failure.sentence);
    }
}
