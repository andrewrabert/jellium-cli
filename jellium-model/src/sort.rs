use jellyfin_api::types::{ItemSortBy, SortOrder};

/// How a browse surface is ordered, shared by the library grid, search results,
/// every hub and every filtered list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sort {
    #[default]
    Name,
    NameDescending,
    DateAdded,
    ReleaseDate,
    CommunityRating,
    Random,
}

impl Sort {
    pub const ALL: [Sort; 6] = [
        Sort::Name,
        Sort::NameDescending,
        Sort::DateAdded,
        Sort::ReleaseDate,
        Sort::CommunityRating,
        Sort::Random,
    ];

    /// The field and direction `/Items` takes.
    pub fn query(self) -> (ItemSortBy, SortOrder) {
        match self {
            Sort::Name => (ItemSortBy::SortName, SortOrder::Ascending),
            Sort::NameDescending => (ItemSortBy::SortName, SortOrder::Descending),
            Sort::DateAdded => (ItemSortBy::DateCreated, SortOrder::Descending),
            Sort::ReleaseDate => (ItemSortBy::PremiereDate, SortOrder::Descending),
            Sort::CommunityRating => (ItemSortBy::CommunityRating, SortOrder::Descending),
            Sort::Random => (ItemSortBy::Random, SortOrder::Ascending),
        }
    }

    /// True for the two name orders, which are the only sorts carrying a letter
    /// jump.
    pub fn by_name(self) -> bool {
        matches!(self, Sort::Name | Sort::NameDescending)
    }

    /// The spelling the preference bag holds this sort under.
    pub fn key(self) -> &'static str {
        match self {
            Sort::Name => "name",
            Sort::NameDescending => "nameDescending",
            Sort::DateAdded => "dateAdded",
            Sort::ReleaseDate => "releaseDate",
            Sort::CommunityRating => "communityRating",
            Sort::Random => "random",
        }
    }

    /// The sort `raw` names, and `None` for text naming none.
    pub fn parse(raw: &str) -> Option<Sort> {
        Sort::ALL.into_iter().find(|sort| sort.key() == raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_sort_parses_from_the_key_it_is_held_under() {
        for sort in Sort::ALL {
            assert_eq!(Sort::parse(sort.key()), Some(sort));
        }
    }

    #[test]
    fn text_naming_no_sort_parses_as_none() {
        assert_eq!(Sort::parse("sideways"), None);
    }

    #[test]
    fn the_two_name_orders_are_the_sorts_carrying_a_letter_jump() {
        let by_name: Vec<Sort> = Sort::ALL
            .into_iter()
            .filter(|sort| sort.by_name())
            .collect();
        assert_eq!(by_name, vec![Sort::Name, Sort::NameDescending]);
    }
}
