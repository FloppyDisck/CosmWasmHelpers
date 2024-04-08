use cosmwasm_std::{to_json_binary, Deps, QueryRequest, StdResult, WasmQuery as WasmQueryMsg};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait WasmQuery: Serialize + Sized {
    type Response: DeserializeOwned;

    fn query(&self, deps: &Deps, address: impl Into<String>) -> StdResult<Self::Response> {
        deps.querier.query(&QueryRequest::Wasm(WasmQueryMsg::Smart {
            contract_addr: address.into(),
            msg: to_json_binary(self)?,
        }))
    }
}
