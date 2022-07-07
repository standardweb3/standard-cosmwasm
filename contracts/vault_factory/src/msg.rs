use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    // vault custom methods
    Liquidate {},
    CreateVault {
        c_denom: String,
        c_amount: Uint128,
        owner: String
    },
    WithdrawCollateral { amount: Uint128 },
    DepositCollateral { amount: Uint128},
    BorrowMore { amount: Uint128 },
    Paydebt { amount: Uint128 },
    CloseVault { amount: Uint128}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
}


