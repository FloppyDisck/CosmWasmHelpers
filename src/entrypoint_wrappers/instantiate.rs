use cosmwasm_std::{to_json_binary, Binary, Coin, CosmosMsg, StdResult, WasmMsg};
use serde::Serialize;

pub trait WasmInstantiate: Serialize + Sized {
    fn instantiate(
        &self,
        code_id: u64,
        label: impl Into<String>,
    ) -> StdResult<WasmInstantiateBuilder> {
        WasmInstantiateBuilder::new(code_id, label, self)
    }
}

pub struct WasmInstantiateBuilder {
    admin: Option<String>,
    code_id: u64,
    msg: Binary,
    funds: Vec<Coin>,
    label: String,
}

impl WasmInstantiateBuilder {
    pub fn new<T: Serialize + Sized>(
        code_id: u64,
        label: impl Into<String>,
        msg: &T,
    ) -> StdResult<Self> {
        Ok(Self {
            admin: None,
            code_id,
            msg: to_json_binary(msg)?,
            funds: Default::default(),
            label: label.into(),
        })
    }

    pub fn with_admin(mut self, admin: impl Into<String>) -> Self {
        self.admin = Some(admin.into());
        self
    }

    pub fn with_funds(mut self, funds: Vec<Coin>) -> Self {
        self.funds = funds;
        self
    }
}

impl Into<CosmosMsg> for WasmInstantiateBuilder {
    fn into(self) -> CosmosMsg {
        CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: self.admin,
            code_id: self.code_id,
            msg: self.msg,
            funds: self.funds,
            label: self.label,
        })
    }
}
