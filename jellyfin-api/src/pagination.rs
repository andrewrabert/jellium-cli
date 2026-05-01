use crate::types;

pub trait Paginated {
    type Item: serde::Serialize;
    fn items(&mut self) -> Vec<Self::Item>;
    fn total_record_count(&self) -> Option<i32>;
}

impl Paginated for types::BaseItemDtoQueryResult {
    type Item = types::BaseItemDto;
    fn items(&mut self) -> Vec<Self::Item> {
        std::mem::take(&mut self.items)
    }
    fn total_record_count(&self) -> Option<i32> {
        self.total_record_count
    }
}

impl Paginated for types::ActivityLogEntryQueryResult {
    type Item = types::ActivityLogEntry;
    fn items(&mut self) -> Vec<Self::Item> {
        std::mem::take(&mut self.items)
    }
    fn total_record_count(&self) -> Option<i32> {
        self.total_record_count
    }
}

impl Paginated for types::RemoteImageResult {
    type Item = types::RemoteImageInfo;
    fn items(&mut self) -> Vec<Self::Item> {
        self.images.take().unwrap_or_default()
    }
    fn total_record_count(&self) -> Option<i32> {
        self.total_record_count
    }
}

impl Paginated for types::SearchHintResult {
    type Item = types::SearchHint;
    fn items(&mut self) -> Vec<Self::Item> {
        std::mem::take(&mut self.search_hints)
    }
    fn total_record_count(&self) -> Option<i32> {
        self.total_record_count
    }
}
