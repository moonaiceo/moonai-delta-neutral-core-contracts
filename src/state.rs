use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Coin, Timestamp};
use cw_storage_plus::{Item, Map};
use injective_cosmwasm::{MarketId, SubaccountId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw20::{AllowanceResponse, Logo, MarketingInfoResponse};

#[cw_serde]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub mint: Option<MinterData>,
}

#[cw_serde]
pub struct MinterData {
    pub minter: Addr,
    /// cap is how many more tokens can be issued by the minter
    pub cap: Option<Uint128>,
}

impl TokenInfo {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractConfigState {
    pub contract_subaccount_id: SubaccountId,
    pub genesis_block_number: u64,
    pub savings_rate: u64,
    pub owner: Addr,
    pub block_generation_time_ms: u64,
}

pub const SWAP_OPERATION_STATE: Item<SwapCacheState> = Item::new("cache");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SwapCacheState {
    pub sender_address: String,
    pub deposited_amount: Coin,
}

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");
// TODO: After https://github.com/CosmWasm/cw-plus/issues/670 is implemented, replace this with a `MultiIndex` over `ALLOWANCES`
pub const ALLOWANCES_SPENDER: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance_spender");
pub const CONFIG_STATE: Item<ContractConfigState> = Item::new("state");

pub const STATE: Item<ContractConfigState> = Item::new("state");
