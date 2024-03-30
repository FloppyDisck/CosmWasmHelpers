mod execute;
mod instantiate;
mod query;

mod prelude {
    pub use crate::execute::WasmExecute;
    pub use crate::instantiate::WasmInstantiate;
    pub use crate::query::WasmQuery;
    pub use cosmwasmhelpers_derive::Execute;
    pub use cosmwasmhelpers_derive::Instantiate;
    pub use cosmwasmhelpers_derive::Query;
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Coin, CosmosMsg, Deps, DepsMut, to_json_binary, WasmMsg};
    use cosmwasm_std::testing::mock_dependencies;
    use super::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Instantiate, Serialize)]
    struct Inst {}

    #[test]
    fn instantiate() {
        let res: CosmosMsg = Inst {}.instantiate(10, "label").unwrap().with_funds(vec![Coin::new(1, "coin")]).with_admin("admin").into();
        assert_eq!(res, CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: Some("admin".into()),
            code_id: 10,
            msg: to_json_binary(&Inst {}).unwrap(),
            funds: vec![Coin::new(1, "coin")],
            label: "label".into(),
        }));
    }

    #[derive(Execute, Serialize)]
    struct Exec {}

    #[test]
    fn execute() {
        let res: CosmosMsg = Exec {}.execute("test").unwrap().with_funds(vec![Coin::new(1, "coin")]).into();
        assert_eq!(res, CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "test".into(),
            msg: to_json_binary(&Exec {}).unwrap(),
            funds: vec![Coin::new(1, "coin")],
        }));
        let other_res: CosmosMsg = Exec {}.execute_with_funds("test", vec![Coin::new(1, "coin")]).unwrap().into();
        assert_eq!(res, other_res);
    }

    #[derive(Query, Serialize)]
    #[Response(QueryResponse)]
    struct QueryMsg {}

    #[derive(Serialize, Deserialize)]
    struct QueryResponse {}
}