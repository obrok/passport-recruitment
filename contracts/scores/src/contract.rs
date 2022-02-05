#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:scores";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetOwner { owner } => try_set_owner(deps, info, owner),
    }
}

pub fn try_set_owner(
    deps: DepsMut,
    info: MessageInfo,
    owner: Addr,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.owner = owner;
        Ok(state)
    })?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse { owner: state.owner })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn get_owner() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();

        assert_eq!(value.owner, Addr::unchecked("creator"));
    }

    #[test]
    fn set_owner() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // the owner can change the owner
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::SetOwner {
            owner: Addr::unchecked("new_owner"),
        };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(value.owner, Addr::unchecked("new_owner"));
    }

    #[test]
    fn set_owner_unauthorized() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // only owner can change the owner
        let auth_info = mock_info("other", &coins(2, "token"));
        let msg = ExecuteMsg::SetOwner {
            owner: Addr::unchecked("new_owner"),
        };
        match execute(deps.as_mut(), mock_env(), auth_info, msg) {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(value.owner, Addr::unchecked("creator"));
    }
}
