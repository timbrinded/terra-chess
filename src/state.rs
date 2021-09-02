use cosmwasm_std::Addr;
use cw_controllers::{Admin, Hooks};
use cw_storage_plus::Map;
use serde::{Deserialize, Serialize};

use crate::engine::Game as ChessGame;
use crate::msg::ChessMatch;
//use chess::Game as ChessGame;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
pub struct ChessMove {
    pub original: (usize, usize),
    pub new: (usize, usize),
}

pub const GAMES: Map<(&Addr, &Addr), ChessMatch> = Map::new("game");
pub const LEADERBOARD: Map<&Addr, u32> = Map::new("leaderboard");
pub const ADMIN: Admin = Admin::new("admin");
pub const HOOKS: Hooks = Hooks::new("hooks");

pub const MATCHS: Map<(&Addr, &Addr), Vec<ChessMove>> = Map::new("match");
