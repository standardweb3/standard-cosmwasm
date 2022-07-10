#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stnd-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<OsmosisQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        vault_id: msg.vault_id,
        manager: msg.manager,
        debt: msg.debt,
        collateral: msg.collateral,
        borrow: msg.borrow,
        last_updated: msg.created_at,
        ex_sfr: Uint128::zero(),
        v1: msg.v1
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Set NFT lock
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("vault_id", msg.vault_id.to_string()))
}

pub mod execute;

pub mod query;