#![allow(unused)]

use crate::prelude::*;

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct SearchResult {
    character: Option<Vec<u32>>,
    constellation: Option<Vec<u32>>,
    corporation: Option<Vec<u32>>,
    solar_system: Option<Vec<u32>>,
    station: Option<Vec<u32>>,
}

/// Endpoints for Search
pub struct SearchGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl<'a> SearchGroup<'a> {
    api_get!(
        /// Get a character's public information.
        get_search,
        "get_search",
        RequestType::Public,
        SearchResult,;
        (categories: &str) => "categories",
        (search: &str) => "search",
        (strict: bool) => "strict"
    );
}