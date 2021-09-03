use crate::msg::{GameMove, GameResult, RPSMatch};
use cosmwasm_std::Addr;
use cw_controllers::Admin;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, JsonSchema)]
pub struct ChessMove {
    pub original: (u8, u8),
    pub new: (u8, u8),
}

pub const ADMIN: Admin = Admin::new("admin");
pub const MATCHS: Map<(&Addr, &Addr), Vec<ChessMove>> = Map::new("match");
pub const GAMES: Map<(&Addr, &Addr), Vec<ChessMove>> = Map::new("game");
