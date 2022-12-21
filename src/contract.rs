#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ArbiterResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:escrow";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        arbiter: deps.api.addr_validate(&msg.arbiter)?,
        recipient: deps.api.addr_validate(&msg.recipient)?,
        source: info.sender,
        expiration: msg.expiration,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if let Some(expiration) = msg.expiration {
        if expiration.is_expired(&env.block) {
            return Err(ContractError::Expired { expiration });
        }
    }
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Approve { quantity } => execute_approve(deps, env, info, quantity),
        ExecuteMsg::Refund {} => execute_refund(deps, env, info),
    }
}

fn execute_approve(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    quantity: Option<Vec<Coin>>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.arbiter {
        return Err(ContractError::Unauthorized {});
    }

    // throws error if the contract is expired
    if let Some(expiration) = config.expiration {
        if expiration.is_expired(&env.block) {
            return Err(ContractError::Expired { expiration });
        }
    }

    let amount = if let Some(quantity) = quantity {
        quantity
    } else {
        // release everything
        // Querier guarantees to return up-to-date data. including funds sent in this handle message
        deps.querier.query_all_balances(&env.contract.address)?
    };

    Ok(send_tokens(config.recipient, amount, "approve"))
}

fn execute_refund(deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // throws error if the contract is expired
    if let Some(expiration) = config.expiration {
        if !expiration.is_expired(&env.block) {
            return Err(ContractError::NotExpired {});
        }
    } else {
        return Err(ContractError::NotExpired {});
    }

    let balance = deps.querier.query_all_balances(env.contract.address)?;

    Ok(send_tokens(config.arbiter, balance, "refund"))
}

fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Arbiter {} => to_binary(&query_arbiter(deps)?),
    }
}

fn query_arbiter(deps: Deps) -> StdResult<ArbiterResponse> {
    let config = CONFIG.load(deps.storage)?;
    let addr = config.arbiter;
    Ok(ArbiterResponse { arbiter: addr })
}

#[cfg(test)]
mod tests {
    use core::panic;

    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, CosmosMsg, Timestamp,
    };
    use cw_utils::Expiration;

    use crate::{
        contract::{execute, query_arbiter},
        msg::{ExecuteMsg, InstantiateMsg},
        state::{Config, CONFIG},
        ContractError,
    };

    use super::instantiate;

    fn init_msg_expire_by_height(expiration: Option<Expiration>) -> InstantiateMsg {
        InstantiateMsg {
            arbiter: String::from("verifies"),
            recipient: String::from("benefits"),
            expiration,
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = init_msg_expire_by_height(Some(Expiration::AtHeight(1000)));
        let mut env = mock_env();
        env.block.height = 876;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let state = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(
            state,
            Config {
                arbiter: Addr::unchecked("verifies"),
                recipient: Addr::unchecked("benefits"),
                source: Addr::unchecked("creator"),
                expiration: Some(Expiration::AtHeight(1000))
            }
        );
    }

    #[test]
    fn cannot_initiate_expired() {
        let mut deps = mock_dependencies();

        let msg = init_msg_expire_by_height(Some(Expiration::AtHeight(1000)));
        let mut env = mock_env();
        env.block.height = 1001;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("creator", &coins(1000, "earch"));

        let res = instantiate(deps.as_mut(), env, info, msg);
        match res.unwrap_err() {
            ContractError::Expired { .. } => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn init_and_query() {
        let mut deps = mock_dependencies();

        let arbiter = Addr::unchecked("arbiters");
        let recipient = Addr::unchecked("receives");
        let creator = Addr::unchecked("creators");
        let msg = InstantiateMsg {
            arbiter: arbiter.clone().into(),
            recipient: recipient.clone().into(),
            expiration: None,
        };
        let mut env = mock_env();
        env.block.height = 876;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info(creator.as_str(), &[]);
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        // now let's query
        let query_response = query_arbiter(deps.as_ref()).unwrap();
        assert_eq!(query_response.arbiter, arbiter);
    }

    #[test]
    fn execute_approve() {
        let mut deps = mock_dependencies();

        // initialize store
        let init_amount = coins(1000, "earth");
        let msg = init_msg_expire_by_height(Some(Expiration::AtHeight(1000)));
        let mut env = mock_env();
        env.block.height = 876;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("creator", &init_amount);
        let contract_addr = env.clone().contract.address;
        let init_res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, init_res.messages.len());

        // balance changed in init
        deps.querier.update_balance(&contract_addr, init_amount);

        // beneficiary cannot release it
        let msg = ExecuteMsg::Approve { quantity: None };
        let mut env = mock_env();
        env.block.height = 900;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("beneficiary", &[]);
        let execute_res = execute(deps.as_mut(), env, info, msg.clone());
        match execute_res.unwrap_err() {
            ContractError::Unauthorized { .. } => {}
            e => panic!("unexpected error: {:?}", e),
        }

        // verifier cannot release it when expired
        let mut env = mock_env();
        env.block.height = 999;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("verifies", &[]);
        let execute_res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, execute_res.messages.len());
        let msg = execute_res.messages.get(0).expect("no message");
        assert_eq!(
            msg.msg,
            CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                to_address: "benefits".into(),
                amount: coins(1000, "earth")
            })
        );

        // partial release by verfier, before expiration
        let partial_msg = ExecuteMsg::Approve {
            quantity: Some(coins(500, "earth")),
        };
        let mut env = mock_env();
        env.block.height = 999;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("verifies", &[]);
        let execute_res = execute(deps.as_mut(), env, info, partial_msg).unwrap();
        assert_eq!(1, execute_res.messages.len());
        let msg = execute_res.messages.get(0).expect("no message");
        assert_eq!(
            msg.msg,
            CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                to_address: "benefits".into(),
                amount: coins(500, "earth")
            })
        );
    }

    #[test]
    fn handle_refund() {
        let mut deps = mock_dependencies();

        // initialize the store
        let init_amount = coins(1000, "earth");
        let msg = init_msg_expire_by_height(Some(Expiration::AtHeight(1000)));
        let mut env = mock_env();
        env.block.height = 876;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("creator", &init_amount);
        let contract_addr = env.clone().contract.address;
        let init_res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, init_res.messages.len());

        // balance changed in init
        deps.querier.update_balance(&contract_addr, init_amount);

        // can not release when unexpired (height < Expiration::AtHeight(1000))
        let msg = ExecuteMsg::Refund {};
        let mut env = mock_env();
        env.block.height = 800;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("anybody", &[]);
        let execute_res = execute(deps.as_mut(), env, info, msg);
        match execute_res.unwrap_err() {
            ContractError::NotExpired {} => {}
            e => panic!("unexpected error: {:?}", e),
        }

        // contract when height  == Expiration::AtHeight(1000)
        let msg = ExecuteMsg::Refund {};
        let mut env = mock_env();
        env.block.height = 1000;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("anybody", &[]);
        let execute_res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, execute_res.messages.len());
        let msg = execute_res.messages.get(0).expect("no message");
        assert_eq!(
            msg.msg,
            CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                to_address: "verifies".into(),
                amount: coins(1000, "earth"),
            })
        );

        // anyone can release after expiration
        let msg = ExecuteMsg::Refund {};
        let mut env = mock_env();
        env.block.height = 1001;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("anybody", &[]);
        let execute_res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, execute_res.messages.len());
        let msg = execute_res.messages.get(0).expect("no message");
        assert_eq!(
            msg.msg,
            CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                to_address: "verifies".into(),
                amount: coins(1000, "earth"),
            })
        );
    }

    #[test]
    fn handle_refund_no_expiration() {
        let mut deps = mock_dependencies();

        // initialize the store
        let init_amount = coins(1000, "earth");
        let msg = init_msg_expire_by_height(None);
        let mut env = mock_env();
        env.block.height = 876;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("creator", &init_amount);
        let contract_addr = env.clone().contract.address;
        let init_res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, init_res.messages.len());

        // balance changed in init
        deps.querier.update_balance(&contract_addr, init_amount);

        // cannot release when unexpired (no expiration)
        let msg = ExecuteMsg::Refund {};
        let mut env = mock_env();
        env.block.height = 800;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("anybody", &[]);
        let execute_res = execute(deps.as_mut(), env, info, msg);
        match execute_res.unwrap_err() {
            ContractError::NotExpired {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }
}
