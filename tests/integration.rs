use cosmwasm_std::{
    coins,
    testing::{mock_info, MOCK_CONTRACT_ADDR},
    Addr, BlockInfo, Coin, Env, MessageInfo, Response, Timestamp, TransactionInfo,
};
use cosmwasm_vm::{
    from_slice,
    testing::{instantiate, mock_instance},
    Storage,
};
use cw_utils::Expiration;
use escrow::{msg::InstantiateMsg, state::Config};

static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/escrow.wasm");

fn init_msg_expire_by_height(expiration: Expiration) -> InstantiateMsg {
    InstantiateMsg {
        arbiter: String::from("verifies"),
        recipient: String::from("benefits"),
        expiration: Some(expiration),
    }
}

fn mock_env_info_height(signer: &str, sent: &[Coin], height: u64, time: u64) -> (Env, MessageInfo) {
    let env = Env {
        block: BlockInfo {
            height,
            time: Timestamp::from_seconds(time),
            chain_id: String::from("test"),
        },
        contract: cosmwasm_std::ContractInfo {
            address: Addr::unchecked(MOCK_CONTRACT_ADDR),
        },
        transaction: Some(TransactionInfo { index: 2 }),
    };

    let info = mock_info(signer, sent);
    (env, info)
}

#[test]
fn propoer_initialization() {
    let mut deps = mock_instance(WASM, &[]);
    let msg = init_msg_expire_by_height(Expiration::AtHeight(1000));
    let (env, info) = mock_env_info_height("creator", &coins(1000, "earth"), 876, 0);

    let res: Response = instantiate(&mut deps, env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps.with_storage(|store| {
        let state: Config = from_slice(&store.get(b"config").0.unwrap().unwrap(), 2048).unwrap();
        assert_eq!(
            state,
            Config {
                arbiter: Addr::unchecked("verifies"),
                recipient: Addr::unchecked("benefits"),
                source: Addr::unchecked("creator"),
                expiration: Some(Expiration::AtHeight(1000)),
            }
        );
        Ok(())
    })
    .unwrap()
}
