//! The stub upstream's areas, mounted together, over the record of what every
//! request carried.

mod dashboard;
mod library;
mod livetv;
mod login;
mod startup;
mod stream;

pub use dashboard::Dashboard;
pub use library::{Entry, Library};
pub use livetv::LiveTv;
pub use login::Login;
pub use startup::Startup;
pub use stream::Stream;

/// Every request the stub upstream took, and whether it carried an
/// `Authorization` header.
#[derive(Clone)]
pub struct Taken {
    held: std::sync::Arc<std::sync::Mutex<Vec<Carried>>>,
}

/// A path asked for and every header the request carried.
type Carried = (String, axum::http::HeaderMap);

impl Taken {
    fn new() -> Taken {
        Taken {
            held: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Records that `path` arrived carrying `headers`.
    pub fn record(&self, path: &str, headers: &axum::http::HeaderMap) {
        self.held
            .lock()
            .expect("the record of what every request carried")
            .push((path.to_owned(), headers.clone()));
    }

    /// Every header the first request to `path` carried, and `None` when
    /// `path` was never asked for.
    pub fn headers(&self, path: &str) -> Option<axum::http::HeaderMap> {
        self.held
            .lock()
            .expect("the record of what every request carried")
            .iter()
            .find(|(asked, _)| asked == path)
            .map(|(_, carried)| carried.clone())
    }

    /// The `Authorization` value the first request to `path` carried, and
    /// `None` when it carried none or `path` was never asked for.
    pub fn authorization(&self, path: &str) -> Option<String> {
        self.headers(path)?
            .get(axum::http::header::AUTHORIZATION)?
            .to_str()
            .ok()
            .map(str::to_owned)
    }

    /// The paths asked for without an `Authorization` header, in arrival
    /// order.
    pub fn tokenless(&self) -> Vec<String> {
        self.held
            .lock()
            .expect("the record of what every request carried")
            .iter()
            .filter(|(_, carried)| !carried.contains_key(axum::http::header::AUTHORIZATION))
            .map(|(path, _)| path.clone())
            .collect()
    }

    /// The paths asked for with one, in arrival order.
    pub fn credentialed(&self) -> Vec<String> {
        self.held
            .lock()
            .expect("the record of what every request carried")
            .iter()
            .filter(|(_, carried)| carried.contains_key(axum::http::header::AUTHORIZATION))
            .map(|(path, _)| path.clone())
            .collect()
    }
}

/// The stub upstream's areas, the record of what every request carried, and
/// the router serving them.
pub struct Synthetic {
    pub router: axum::Router,
    pub live_tv: LiveTv,
    pub dashboard: Dashboard,
    pub startup: Startup,
    pub login: Login,
    pub library: Library,
    pub taken: Taken,
}

pub fn router() -> Synthetic {
    let (live_tv_router, live_tv) = livetv::router();
    let (dashboard_router, dashboard) = dashboard::router();
    let (startup_router, startup) = startup::router();
    let (login_router, login) = login::router();
    let (library_router, library) = library::router();
    let taken = Taken::new();
    let router = live_tv_router
        .merge(dashboard_router)
        .merge(startup_router)
        .merge(login_router)
        .merge(library_router)
        .merge(stream::router())
        .layer(axum::middleware::from_fn_with_state(
            taken.clone(),
            recording,
        ));
    Synthetic {
        router,
        live_tv,
        dashboard,
        startup,
        login,
        library,
        taken,
    }
}

/// Records every request the stub's own routes took before serving it.
pub async fn recording(
    axum::extract::State(taken): axum::extract::State<Taken>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let path = request.uri().path().to_owned();
    taken.record(&path, request.headers());
    next.run(request).await
}
