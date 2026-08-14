use uuid::Uuid;

use crate::screen::library::Sort;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Home,
    Library { id: Uuid, sort: Sort, start: i32 },
    Detail { id: Uuid },
    Search { term: String, start: i32 },
}
