use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub arbiter: String,
    pub recipient: String,
    pub expiration: Option<Expiration>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Approve { quantity: Option<Vec<Coin>> },
    Refund {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ArbiterResponse)]
    Arbiter {},
}

#[cw_serde]
pub struct ArbiterResponse {
    pub arbiter: Addr,
}
