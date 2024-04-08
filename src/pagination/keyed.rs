use crate::pagination::{PageOrder, Pagination};
use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::{Bound, KeyDeserialize, Map, PrimaryKey};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
/// Pagination based queries should return this type for easier frontend pagination
pub struct KeyedPageResult<T, P> {
    /// Requested data
    pub data: Vec<T>,
    /// Current page
    pub page: P,
    // Next page
    pub next: Option<P>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
/// Pagination based queries should use this type for easier query handling
pub struct KeyedPageRequest<K> {
    // Page to get next, if none start from 0
    pub page: Option<K>,
    // Item limit for pages
    pub limit: Option<u32>,
    // Request ordering
    pub order: Option<PageOrder>,
}

impl<'map, 'key, K, V, P, R> Pagination<'map, 'key, K, V, P, R> for KeyedPageRequest<K>
    where
        'map: 'key,
        K: PrimaryKey<'key> + KeyDeserialize<Output=P>,
        V: Serialize + DeserializeOwned,
        P: Clone + 'static,
{
    type QueryPageResult = KeyedPageResult<R, P>;
    type KeysPageResult = KeyedPageResult<P, P>;

    fn custom_query<F>(
        self,
        default_limit: u32,
        max_limit: u32,
        storage: &dyn Storage,
        map: &'map Map<K, V>,
        mapper: F,
    ) -> StdResult<Self::QueryPageResult>
        where
            F: FnMut(&(P, V)) -> R,
    {
        let limit = self.limit.unwrap_or(default_limit).min(max_limit) as usize;
        let order = self.order.unwrap_or(PageOrder::Ascending);

        let iterator = map.range(
            storage,
            self.page.map(|k| Bound::inclusive(k)),
            None,
            order.into(),
        );
        let mut result: Vec<(P, V)> = iterator
            .take(limit + 1)
            .collect::<StdResult<Vec<(P, V)>>>()?;
        let current = result.get(0).unwrap().0.clone();
        let next = if result.len() - 1 == limit {
            result.pop().map(|(k, _)| k)
        } else {
            None
        };

        Ok(KeyedPageResult {
            data: result.iter().map(mapper).collect(),
            page: current,
            next,
        })
    }

    fn custom_keys(
        self,
        default_limit: u32,
        max_limit: u32,
        storage: &dyn Storage,
        map: &'map Map<K, V>,
    ) -> StdResult<Self::KeysPageResult>
        where
            K: PrimaryKey<'key> + KeyDeserialize<Output=P>,
    {
        let limit = self.limit.unwrap_or(default_limit).min(max_limit) as usize;
        let order = self.order.unwrap_or(PageOrder::Ascending);

        let iterator = map.keys(
            storage,
            self.page.map(|k| Bound::inclusive(k)),
            None,
            order.into(),
        );
        let mut result: Vec<P> = iterator.take(limit + 1).collect::<StdResult<Vec<P>>>()?;
        let current = result.get(0).unwrap().clone();
        let next = if result.len() - 1 == limit {
            result.pop()
        } else {
            None
        };

        Ok(KeyedPageResult {
            data: result,
            page: current,
            next,
        })
    }
}