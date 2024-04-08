use crate::pagination::{PageOrder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
/// Pagination based queries should return this type for easier frontend pagination
pub struct NumberedPageResult<T> {
    /// Requested data
    pub data: Vec<T>,
    /// Current page
    pub page: u64,
    // Total items on storage
    pub total: u64,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
/// Pagination based queries should use this type for easier query handling
pub struct NumberedPageRequest<K> {
    // Page to get next, if none start from 0
    pub page: Option<K>,
    // Item limit for pages
    pub limit: Option<u32>,
    // Request ordering
    pub order: Option<PageOrder>,
}