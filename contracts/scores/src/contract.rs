#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg, ScoreResponse};
use crate::state::{State, STATE};

use std::collections::HashMap;

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
        scores: HashMap::new(),
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
        ExecuteMsg::SetScore { addr, score } => try_set_score(deps, info, addr, score),
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

pub fn try_set_score(
    deps: DepsMut,
    info: MessageInfo,
    addr: Addr,
    score: i32,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.scores.insert(addr, score);
        Ok(state)
    })?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetScore { addr } => to_binary(&query_score(deps, addr)?),
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse { owner: state.owner })
}

fn query_score(deps: Deps, addr: Addr) -> StdResult<ScoreResponse> {
    let state = STATE.load(deps.storage)?;
    match state.scores.get(&addr) {
        Some(score) => Ok(ScoreResponse::Success {
            addr,
            score: score.clone(),
        }),
        None => Ok(ScoreResponse::UnknownAddress {}),
    }
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

    #[test]
    fn set_score() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // the owner can change scores
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::SetScore {
            addr: Addr::unchecked("address1"),
            score: 123,
        };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetScore {
                addr: Addr::unchecked("address1"),
            },
        )
        .unwrap();
        let value: ScoreResponse = from_binary(&res).unwrap();
        match value {
            ScoreResponse::Success { addr, score } => {
                assert_eq!(addr, Addr::unchecked("address1"));
                assert_eq!(score, 123);
            }
            _ => panic!("Expected Success"),
        }
    }

    #[test]
    fn get_score_for_unknown_address() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetScore {
                addr: Addr::unchecked("address1"),
            },
        )
        .unwrap();
        let value: ScoreResponse = from_binary(&res).unwrap();
        match value {
            ScoreResponse::UnknownAddress {} => {}
            _ => panic!("Expected UnknownAddress"),
        }
    }

    #[test]
    fn set_score_unauthorized() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // only owner can change scores
        let auth_info = mock_info("other", &coins(2, "token"));
        let msg = ExecuteMsg::SetScore {
            addr: Addr::unchecked("address1"),
            score: 123,
        };
        match execute(deps.as_mut(), mock_env(), auth_info, msg) {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetScore {
                addr: Addr::unchecked("address1"),
            },
        )
        .unwrap();
        let value: ScoreResponse = from_binary(&res).unwrap();
        match value {
            ScoreResponse::UnknownAddress {} => {}
            _ => panic!("Expected UnknownAddress"),
        }
    }
}
