#![allow(unused_imports)]
//#![allow(dead_code)]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
};
use std::result::Result;

use crate::engine::Game as ChessGame;
use crate::engine::VictoryStatus;
use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;
use cw0::maybe_addr;
use serde::{Deserialize, Serialize};

//#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;
    ADMIN.set(deps.branch(), maybe_addr(api, msg.admin)?)?;
    Ok(Response::default())
}

//#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;
    match msg {
        ExecuteMsg::UpdateAdmin { admin } => {
            Ok(ADMIN.execute_update_admin(deps, info, maybe_addr(api, admin)?)?)
        }
        ExecuteMsg::StartMatch {
            opponent,
            first_move,
        } => try_start_match(deps, info, opponent, first_move),
        ExecuteMsg::PlayMove {
            host,
            opponent,
            your_move,
        } => try_make_move(deps, info, host, opponent, your_move),
    }
}

pub fn try_make_move(
    deps: DepsMut,
    info: MessageInfo,
    host: String,
    opponent: String,
    your_move: ChessMove,
) -> Result<Response, ContractError> {
    let host_checked = deps.api.addr_validate(&host)?;
    let opponent_checked = deps.api.addr_validate(&opponent)?;
    let mut game = ChessGame::new();

    let mut moves_made = MATCHS.load(deps.storage, (&host_checked, &opponent_checked))?;

    for x in &moves_made {
        let pos_start = x.original;
        let pos_end = x.new;
        game.move_piece(pos_start, pos_end);
    }
    // Game state now rebuilt

    let pos_start = your_move.original;
    let pos_end = your_move.new;
    let valid_moves = game.valid_moves(pos_start);
    for i in &valid_moves {
        let (a, b) = i.last().unwrap();
        if b == &pos_end {
            game.move_piece(pos_start, pos_end);
            moves_made.push(your_move);
        }
    }

    match game.check_victory() {
        Some(_) => MATCHS.remove(deps.storage, (&host_checked, &opponent_checked)),
        None => MATCHS.save(
            deps.storage,
            (&host_checked, &opponent_checked),
            &moves_made,
        )?,
    };

    Ok(Response::new())
}

pub fn try_start_match(
    deps: DepsMut,
    info: MessageInfo,
    opponent: String,
    first_move: ChessMove,
) -> Result<Response, ContractError> {
    let host = info.sender;
    let opponent_checked = deps.api.addr_validate(&opponent)?;
    let moves = vec![first_move];

    MATCHS.save(deps.storage, (&host, &opponent_checked), &moves);

    Ok(Response::new())
}

//#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHostGames { host } => to_binary(&query_host_games(deps, host)?),
        QueryMsg::GetAdmin {} => to_binary(&ADMIN.query_admin(deps)?),
        QueryMsg::GetWins { player } => to_binary(&query_board(deps, player)?),
        QueryMsg::CheckMatch { host, opponent } => to_binary(&query_match(deps, host, opponent)?),
    }
}

fn query_match(deps: Deps, host: String, opponent: String) -> StdResult<Vec<ChessMove>> {
    let host_checked = deps.api.addr_validate(&host)?;
    let opponent_checked = deps.api.addr_validate(&opponent)?;
    let match_details = MATCHS.load(deps.storage, (&host_checked, &opponent_checked))?;

    Ok(match_details)
}

fn query_board(deps: Deps, player: String) -> StdResult<u32> {
    let player_checked = deps.api.addr_validate(&player)?;
    let wins = LEADERBOARD.load(deps.storage, &player_checked)?;

    Ok(wins)
}

fn query_host_games(deps: Deps, host: String) -> StdResult<GameList> {
    let host_checked = deps.api.addr_validate(&host)?;
    let games: StdResult<Vec<ChessMatch>> = GAMES
        .prefix(&host_checked)
        .range(deps.storage, None, None, Order::Ascending)
        .take(5)
        .map(|item| {
            let (k, v) = item?;
            Ok(ChessMatch {
                host: host_checked.to_string(),
                opponent: String::from_utf8(k)?,
                host_move: v.host_move,
                opp_move: v.opp_move,
                result: v.result,
            })
        })
        .collect();
    Ok(GameList { games: games? })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ContractError;
    use crate::state::ChessMove;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use cw_controllers::AdminResponse;

    const INIT_ADMIN: &str = "juan";

    #[test]
    fn humble_chess_test() {
        //let mut game = ChessGame::new();
        let mut deps = mock_dependencies(&[]);

        let opening = ChessMove {
            original: (3, 1),
            new: (3, 3),
        };
        let info = mock_info("mario", &coins(1000, "coins"));
        let opponent = String::from("bowser");
        let msg = ExecuteMsg::StartMatch {
            opponent: opponent,
            first_move: opening,
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("bowser", &coins(1000, "coins"));
        let host = String::from("mario");
        let mov = ChessMove {
            original: (4, 6),
            new: (4, 4),
        };
        let msg = ExecuteMsg::PlayMove {
            host: host,
            opponent: info.sender.to_string(),
            your_move: mov,
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("mario", &coins(1000, "coins"));
        let opponent = String::from("bowser");
        let msg = QueryMsg::CheckMatch {
            opponent: opponent,
            host: info.sender.to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let decoded: Vec<ChessMove> = from_binary(&res).unwrap();
        println!("The current game looks like this nonsense:");
        println!("{:?}", decoded);
    }
}
