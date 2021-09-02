#![allow(unused_imports)]
use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use cw_controllers::{Admin, Hooks};
use crate::msg::{ChessMatch};


pub const GAMES: Map<(&Addr, &Addr), ChessMatch> = Map::new("game");
pub const LEADERBOARD: Map<&Addr, u32> = Map::new("leaderboard");
pub const ADMIN: Admin = Admin::new("admin");
pub const HOOKS: Hooks = Hooks::new("hooks");
