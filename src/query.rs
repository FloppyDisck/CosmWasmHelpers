use cosmwasm_std::{Deps, QueryRequest, StdResult, to_json_binary, WasmQuery as WasmQueryMsg};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait WasmQuery: Serialize + Sized {
    type Response: DeserializeOwned;
    
    fn query(&self, deps: &Deps, address: impl Into<String>) -> StdResult<Self::Response> {
        deps.querier.query(&QueryRequest::Wasm(WasmQueryMsg::Smart {
            contract_addr: address.into(),
            msg: to_json_binary(self)?
        }))
    }
}