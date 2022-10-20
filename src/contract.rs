#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, NumberResponse};
use crate::state::PHONEBOOK;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:phonebook";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn execute_add_number(
    deps: DepsMut,
    info: MessageInfo,
    number: String,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone();
    if PHONEBOOK.has(deps.storage, sender.clone()) {
        Err(ContractError::DuplicateKey)
    } else {
        match PHONEBOOK.save(deps.storage, sender, &number) {
            Ok(_) => {
                let res = Response::new()
                    .add_attribute("action", "add_number")
                    .add_attribute("sender", info.sender)
                    .add_attribute("number", number);
                Ok(res)
            }
            Err(e) => Err(ContractError::Std(e)),
        }
    }
}

fn execute_remove_number(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let sender = info.sender.clone();
    if PHONEBOOK.has(deps.storage, sender.clone()) {
        PHONEBOOK.remove(deps.storage, sender);
        let res = Response::new()
            .add_attribute("action", "remove_number")
            .add_attribute("sender", info.sender);
        Ok(res)
    } else {
        Err(ContractError::No)
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match &msg {
        ExecuteMsg::AddNumber { number } => execute_add_number(deps, info, number.to_string()),
        ExecuteMsg::RemoveNumber => execute_remove_number(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNumber { address } => to_binary(&get_number(deps, address)?),
    }
}

fn get_number(deps: Deps, address: String) -> StdResult<NumberResponse> {
    let address = deps.api.addr_validate(&address)?;
    let number = PHONEBOOK
        .may_load(deps.storage, address)?
        .unwrap_or_default();
    Ok(NumberResponse {number})
}

#[cfg(test)]
mod tests {}
