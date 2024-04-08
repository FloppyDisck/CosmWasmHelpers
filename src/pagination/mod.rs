use cosmwasm_std::{Order, StdResult, Storage};
use cw_storage_plus::{KeyDeserialize, Map, PrimaryKey};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod keyed;
mod numbered;

pub use keyed::*;
pub use numbered::*;

// TODO: add tests

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
// Order doesn't have serialization nor deserialization support so we create a wrapper
pub enum PageOrder {
    #[default]
    Ascending,
    Descending,
}

impl Into<Order> for PageOrder {
    fn into(self) -> Order {
        match self {
            PageOrder::Ascending => Order::Ascending,
            PageOrder::Descending => Order::Descending,
        }
    }
}

pub trait Pagination<'map, 'key, K, V, P, R>
    where
        Self: Sized,
        'map: 'key,
        K: PrimaryKey<'key> + KeyDeserialize<Output=P>,
        V: Serialize + DeserializeOwned,
        P: Clone + 'static,
{
    type QueryPageResult;
    type KeysPageResult;

    #[inline]
    fn default_limit() -> u32 {
        10
    }
    #[inline]
    fn max_limit() -> u32 {
        100
    }

    fn custom_query<F>(
        self,
        default_limit: u32,
        max_limit: u32,
        storage: &dyn Storage,
        map: &'map Map<K, V>,
        mapper: F,
    ) -> StdResult<Self::QueryPageResult>
        where
            F: FnMut(&(P, V)) -> R;

    fn query<F>(
        self,
        storage: &dyn Storage,
        map: &'map Map<K, V>,
        mapper: F,
    ) -> StdResult<Self::QueryPageResult>
        where
            F: FnMut(&(P, V)) -> R,
    {
        self.custom_query(
            Self::default_limit(),
            Self::default_limit(),
            storage,
            map,
            mapper,
        )
    }

    fn custom_keys(
        self,
        default_limit: u32,
        max_limit: u32,
        storage: &dyn Storage,
        map: &'map Map<K, V>,
    ) -> StdResult<Self::KeysPageResult>
        where
            K: PrimaryKey<'key> + KeyDeserialize<Output=P>;

    fn keys(self, storage: &dyn Storage, map: &'map Map<K, V>) -> StdResult<Self::KeysPageResult>
        where
            P: Clone + 'static,
            V: Serialize + DeserializeOwned,
            K: PrimaryKey<'key> + KeyDeserialize<Output=P>,
    {
        self.custom_keys(Self::default_limit(), Self::max_limit(), storage, map)
    }
}
