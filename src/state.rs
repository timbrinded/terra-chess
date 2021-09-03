use cosmwasm_std::Addr;
use cw_controllers::Admin;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, JsonSchema)]
pub struct ChessMove {
    pub original: (usize, usize),
    pub new: (usize, usize),
}

pub const ADMIN: Admin = Admin::new("admin");
pub const MATCHS: Map<(&Addr, &Addr), Vec<ChessMove>> = Map::new("match");
