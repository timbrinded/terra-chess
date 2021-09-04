#![allow(clippy::many_single_char_names)]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use std::result::Result;

use crate::engine::Game as ChessGame;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ChessMove, ADMIN, MATCHS};
use cw0::maybe_addr;

#[cfg_attr(not(feature = "library"), entry_point)]
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

#[cfg_attr(not(feature = "library"), entry_point)]
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
    _info: MessageInfo,
    host: String,
    opponent: String,
    your_move: ChessMove,
) -> Result<Response, ContractError> {
    let host_checked = deps.api.addr_validate(&host)?;
    let opponent_checked = deps.api.addr_validate(&opponent)?;
    let mut game = ChessGame::new();

    let mut moves_made = MATCHS.load(deps.storage, (&host_checked, &opponent_checked))?;

    for x in &moves_made {
        let (u, v) = x.original;
        let (w, z) = x.new;
        let pos_start = (u as usize, v as usize);
        let pos_end = (w as usize, z as usize);
        game.move_piece(pos_start, pos_end);
    }
    // Game state now rebuilt

    let (u, v) = your_move.original;
    let (w, z) = your_move.new;
    let pos_start = (u as usize, v as usize);
    let pos_end = (w as usize, z as usize);
    let valid_moves = game.valid_moves(pos_start);
    for i in &valid_moves {
        let (_a, b) = i.last().unwrap();
        if b == &pos_end {
            game.move_piece(pos_start, pos_end);
            moves_made.push(your_move);
        };
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

    MATCHS.save(deps.storage, (&host, &opponent_checked), &moves)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdmin {} => to_binary(&ADMIN.query_admin(deps)?),
        QueryMsg::CheckMatch { host, opponent } => to_binary(&query_match(deps, host, opponent)?),
    }
}

fn query_match(deps: Deps, host: String, opponent: String) -> StdResult<Vec<String>> {
    let host_checked = deps.api.addr_validate(&host)?;
    let opponent_checked = deps.api.addr_validate(&opponent)?;
    let match_details = MATCHS.load(deps.storage, (&host_checked, &opponent_checked))?;
    let mut string = Vec::<String>::new();

    for item in match_details {
        let (x, y) = item.original;
        let (w, v) = item.new;
        let line = String::from("Move made from (")
            + &x.to_string()
            + &",".to_owned()
            + &y.to_string()
            + &") to (".to_owned()
            + &w.to_string()
            + &",".to_owned()
            + &v.to_string()
            + &")".to_owned();
        string.push(line);
    }

    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ChessMove;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

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
        let decoded: Vec<String> = from_binary(&res).unwrap();
        println!("{:?}", decoded);
    }
}
