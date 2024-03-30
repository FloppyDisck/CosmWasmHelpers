use cosmwasm_std::{Binary, Coin, CosmosMsg, StdResult, to_json_binary, WasmMsg};
use serde::Serialize;

pub trait WasmExecute: Serialize + Sized {
    fn execute(&self, addr: impl Into<String>) -> StdResult<WasmExecuteBuilder> {
        WasmExecuteBuilder::new(addr, self)
    }

    fn execute_with_funds(&self, addr: impl Into<String>, funds: Vec<Coin>) -> StdResult<WasmExecuteBuilder> {
        WasmExecuteBuilder::new(addr, self).map(|res| res.with_funds(funds))
    }
}

// Recreated the execute msg in order to have a builder pattern
pub struct WasmExecuteBuilder {
    addr: String,
    msg: Binary,
    funds: Vec<Coin>
}

impl WasmExecuteBuilder {
    pub fn new<T: Serialize + Sized>(addr: impl Into<String>, msg: &T) -> StdResult<Self> {
        Ok(Self {
            addr: addr.into(),
            msg: to_json_binary(msg)?,
            funds: Default::default(),
        })
    }

    pub fn with_funds(mut self, funds: Vec<Coin>) -> Self {
        self.funds = funds;
        self
    }
}

impl Into<CosmosMsg> for WasmExecuteBuilder {
    fn into(self) -> CosmosMsg {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.addr,
            msg: self.msg,
            funds: self.funds
        })
    }
}

