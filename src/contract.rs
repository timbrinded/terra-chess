#![allow(unused_imports)]
//#![allow(dead_code)]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
};
use std::result::Result;

use crate::error::ContractError;
use crate::msg::{
    ChessMatch, ExecuteMsg, GameList, GameMove, GameReponse, GameResult, InstantiateMsg, QueryMsg,
};
use crate::state::{ADMIN, GAMES, HOOKS, LEADERBOARD};
use chess::*;
use cw0::maybe_addr;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
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
        ExecuteMsg::StartGame {
            opponent,
            start_move,
        } => try_start_game(deps, info, opponent, start_move),
        ExecuteMsg::UpdateAdmin { admin } => {
            Ok(ADMIN.execute_update_admin(deps, info, maybe_addr(api, admin)?)?)
        }
        ExecuteMsg::AddHook { addr } => {
            Ok(HOOKS.execute_add_hook(&ADMIN, deps, info, api.addr_validate(&addr)?)?)
        }
        ExecuteMsg::RemoveHook { addr } => {
            Ok(HOOKS.execute_remove_hook(&ADMIN, deps, info, api.addr_validate(&addr)?)?)
        }
        ExecuteMsg::Respond { host, opp_move } => try_respond_game(deps, info, host, opp_move),
    }
}

pub fn try_respond_game(
    deps: DepsMut,
    info: MessageInfo,
    host: String,
    opp_move: GameMove,
) -> Result<Response, ContractError> {
    let opponent = info.sender;
    let host_checked = deps.api.addr_validate(&host)?;
    let game = GAMES.load(deps.storage, (&host_checked, &opponent))?;

    println!("Retrieved match details are: {:?}", game);

    if game.opponent != opponent {
        return Err(ContractError::Unexplained {});
    };

    let host_move = game.host_move;
    let opp_move = opp_move;
    let end = resolve_game(host_move, opp_move);
    println!("Match has now concluded with: {:?}", end);

    match end {
        GameResult::Win => {
            let retrieve = LEADERBOARD.load(deps.storage, &host_checked);
            let wins: u32;
            match retrieve {
                Ok(x) => wins = x + 1,
                Err(_) => wins = 1,
            };
            LEADERBOARD.save(deps.storage, &host_checked, &wins);
        }
        GameResult::Loss => {
            let retrieve = LEADERBOARD.load(deps.storage, &opponent);
            let wins: u32;
            match retrieve {
                Ok(x) => wins = x + 1,
                Err(_) => wins = 1,
            };
            LEADERBOARD.save(deps.storage, &opponent, &wins);
        }
        _ => (),
    }

    GAMES.remove(deps.storage, (&host_checked, &opponent));

    Ok(Response::new())
}

// pub fn update_board(result: GameResult, opponent: String, host: String) -> () {
//     match result{
//         GameResult::Win => LEADERBOARD.save(deps.storage),
//         GameResult::Loss => ,
//         _ => (),
//     }
// }

pub fn resolve_game(host_move: GameMove, opp_move: GameMove) -> GameResult {
    if host_move == opp_move {
        return GameResult::Tie;
    };
    if opp_move == GameMove::NotPlayed {
        return GameResult::Tie;
    };

    match host_move {
        GameMove::Scissors => {
            if opp_move == GameMove::Rock {
                GameResult::Loss
            } else {
                GameResult::Win
            }
        }
        GameMove::Rock => {
            if opp_move == GameMove::Paper {
                GameResult::Loss
            } else {
                GameResult::Win
            }
        }
        GameMove::Paper => {
            if opp_move == GameMove::Scissors {
                GameResult::Loss
            } else {
                GameResult::Win
            }
        }
        GameMove::NotPlayed => GameResult::Tie,
    }
}

pub fn try_start_game(
    deps: DepsMut,
    info: MessageInfo,
    opponent: String,
    start_move: GameMove,
) -> Result<Response, ContractError> {
    let host = info.sender;
    let checked = deps.api.addr_validate(&opponent)?;
    let hooks = HOOKS.query_hooks(deps.as_ref()).unwrap();
    let game = ChessMatch {
        host_move: start_move,
        opp_move: GameMove::NotPlayed,
        result: GameResult::InProgress,
        opponent: checked.to_string(),
        host: host.to_string(),
    };

    if !hooks.hooks.is_empty() {
        for x in &hooks.hooks {
            if x == &host.to_string() {
                return Err(ContractError::Blacklisted {});
            };
        }
    }
    GAMES.save(deps.storage, (&host, &checked), &game)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHostGames { host } => to_binary(&query_host_games(deps, host)?),
        QueryMsg::GetGame { host, opponent } => to_binary(&query_game(deps, host, opponent)?),
        QueryMsg::GetAdmin {} => to_binary(&ADMIN.query_admin(deps)?),
        QueryMsg::GetWins { player } => to_binary(&query_board(deps, player)?),
    }
}

fn query_board(deps: Deps, player: String) -> StdResult<u32> {
    let player_checked = deps.api.addr_validate(&player)?;
    let wins = LEADERBOARD.load(deps.storage, &player_checked)?;

    Ok(wins)
}

fn query_game(deps: Deps, host: String, opponent: String) -> StdResult<GameReponse> {
    let host_checked = deps.api.addr_validate(&host)?;
    let oppo_checked = deps.api.addr_validate(&opponent)?;

    let game = GAMES.load(deps.storage, (&host_checked, &oppo_checked))?;
    let resp = GameReponse {
        host_move: game.host_move,
        opp_move: game.opp_move,
        result: game.result,
    };

    Ok(resp)
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
    use chess::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use cw_controllers::AdminResponse;

    const INIT_ADMIN: &str = "juan";

    //   #[test]
    fn start_game_validates() {
        let mut deps = mock_dependencies(&[]);

        //Call start_game to check address
        let oppo = String::from("badgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadgerbadger");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Scissors,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        let error = ContractError::Std(StdError::GenericErr {
            msg: String::from("Invalid input: human address too long"),
        });
        assert_eq!(error, res);

        //Test valid addresses pass OK
        let oppo = String::from("badger");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Rock,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(Response::new(), res);

        // Test game lookup
        let host = String::from("mario");
        let oppo = String::from("badger");
        let msg = QueryMsg::GetGame {
            host: host,
            opponent: oppo,
        };
        let resp = query(deps.as_ref(), mock_env(), msg).unwrap();
        let game: GameReponse = from_binary(&resp).unwrap();

        assert_eq!(game.result, GameResult::InProgress);
        assert_eq!(game.host_move, GameMove::Rock);
    }

    //  #[test]
    fn multi_game_test() {
        let mut deps = mock_dependencies(&[]);

        // Game1
        let oppo = String::from("luigi");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Paper,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        // Game 2
        let oppo = String::from("peach");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Rock,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Lookup
        let host = String::from("mario");
        let oppo = String::from("luigi");
        let msg = QueryMsg::GetGame {
            host: host,
            opponent: oppo,
        };
        let resp = query(deps.as_ref(), mock_env(), msg).unwrap();
        let game: GameReponse = from_binary(&resp).unwrap();
        assert_eq!(game.host_move, GameMove::Paper);

        // multiquery
        let host = String::from("mario");
        let msg = QueryMsg::GetHostGames { host: host };
        let resp = query(deps.as_ref(), mock_env(), msg).unwrap();
        let games: GameList = from_binary(&resp).unwrap();
        //println!("{:?}",games);
        assert_eq!(games.games[0].opponent, String::from("luigi"));
        assert_eq!(games.games[1].opponent, String::from("peach"));
        assert_eq!(games.games[0].host_move, GameMove::Paper);
    }

    //  #[test]
    fn admin_test() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            admin: Some(INIT_ADMIN.into()),
        };
        let info = mock_info("creator", &[]);

        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Checks that update can be done with admin caller, panics if unwraps Err
        let info = mock_info("juan", &[]);
        let new_admin = Some(String::from("timbo"));
        let msg = ExecuteMsg::UpdateAdmin { admin: new_admin };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("juan", &[]);
        let msg = QueryMsg::GetAdmin {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let admin: AdminResponse = from_binary(&res).unwrap();
        //println!("Admin is: {:?}",admin.admin);
        assert_eq!(Some("timbo".to_string()), admin.admin);

        // Checks that update cannot be done with non-admin caller, panics if unwraps Ok
        let info = mock_info("bob", &[]);
        let new_admin = Some(String::from("jimbo"));
        let msg = ExecuteMsg::UpdateAdmin { admin: new_admin };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        let info = mock_info("bob", &[]);
        let msg = QueryMsg::GetAdmin {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let admin: AdminResponse = from_binary(&res).unwrap();
        // println!("Admin is: {:?}",admin.admin);
        assert_ne!(Some("jimbo".to_string()), admin.admin);
    }

    // #[test]
    fn blacklist_test() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            admin: Some(INIT_ADMIN.into()),
        };
        let info = mock_info("juan", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("juan", &[]);
        let person = String::from("pariah");
        let msg = ExecuteMsg::AddHook { addr: person };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("juan", &[]);
        let person = String::from("outcast");
        let msg = ExecuteMsg::AddHook { addr: person };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let oppo = String::from("luigi");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Paper,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Throw error when blacklisted sender tries to start game
        let oppo = String::from("peach");
        let info = mock_info("pariah", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Paper,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(ContractError::Blacklisted {}, res);
    }

    // #[test]
    fn respond_to_game() {
        let mut deps = mock_dependencies(&[]);
        let host = String::from("mario");
        let oppo = String::from("luigi");

        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Paper,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Check game exists
        let host = String::from("mario");
        let oppo = String::from("luigi");
        let msg = QueryMsg::GetGame {
            host: host,
            opponent: oppo,
        };
        query(deps.as_ref(), mock_env(), msg).unwrap();

        // Respond to game with a win
        let host = String::from("mario");
        let info = mock_info("luigi", &coins(1000, "earth"));
        let msg = ExecuteMsg::Respond {
            host: host,
            opp_move: GameMove::Rock,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Check game has been deleted from state
        let host = String::from("mario");
        let oppo = String::from("luigi");
        let msg = QueryMsg::GetGame {
            host: host,
            opponent: oppo,
        };
        query(deps.as_ref(), mock_env(), msg).unwrap_err();

        // Check leaderboard has been updated
        let host = String::from("mario");
        let msg = QueryMsg::GetWins { player: host };
        let resp = query(deps.as_ref(), mock_env(), msg).unwrap();
        let wins: u32 = from_binary(&resp).unwrap();
        assert_eq!(1, wins);
    }

    // #[test]
    fn multi_games_leaderboard() {
        let mut deps = mock_dependencies(&[]);
        let host = String::from("mario");

        //Game1 - Host wins
        let oppo = String::from("luigi");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Paper,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let host = String::from("mario");
        let info = mock_info("luigi", &coins(1000, "earth"));
        let msg = ExecuteMsg::Respond {
            host: host,
            opp_move: GameMove::Rock,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Game2 - Host wins
        let oppo = String::from("luigi");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Scissors,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let host = String::from("mario");
        let info = mock_info("luigi", &coins(1000, "earth"));
        let msg = ExecuteMsg::Respond {
            host: host,
            opp_move: GameMove::Paper,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Game3 - Opponent wins
        let oppo = String::from("luigi");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Paper,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let host = String::from("mario");
        let info = mock_info("luigi", &coins(1000, "earth"));
        let msg = ExecuteMsg::Respond {
            host: host,
            opp_move: GameMove::Scissors,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Game3 - Game tie
        let oppo = String::from("luigi");
        let info = mock_info("mario", &coins(1000, "earth"));
        let msg = ExecuteMsg::StartGame {
            opponent: oppo,
            start_move: GameMove::Paper,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let host = String::from("mario");
        let info = mock_info("luigi", &coins(1000, "earth"));
        let msg = ExecuteMsg::Respond {
            host: host,
            opp_move: GameMove::Paper,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Check leaderboard has been updated correctly
        let host = String::from("mario");
        let msg = QueryMsg::GetWins { player: host };
        let resp = query(deps.as_ref(), mock_env(), msg).unwrap();
        let wins: u32 = from_binary(&resp).unwrap();
        assert_eq!(2, wins);

        let host = String::from("luigi");
        let msg = QueryMsg::GetWins { player: host };
        let resp = query(deps.as_ref(), mock_env(), msg).unwrap();
        let wins: u32 = from_binary(&resp).unwrap();
        assert_eq!(1, wins);
    }

    #[test]
    fn timbo_chess_test_1() {
        let mut game = Game::new();

        let pos_start = (0, 1);
        let pos_end = (0, 3);

        let valid_moves = game.valid_moves((pos_start));
        //println!("The valid moves are: {:?}", valid_moves);
        for i in &valid_moves {
            let (a, b) = i.last().unwrap();
            //println!("{:?}",b);
            if b == &pos_end {
                println!("The move is valid!");
                game.move_piece((pos_start), (pos_end));
            }
        }

        let string = game.board_to_string(true);
        println!("{}", string);

        let pos_start = (1, 6);
        let pos_end = (1, 4);

        let valid_moves = game.valid_moves((pos_start));
        //println!("The valid moves are: {:?}", valid_moves);
        for i in &valid_moves {
            let (a, b) = i.last().unwrap();
            //println!("{:?}",b);
            if b == &pos_end {
                println!("The move is valid!");
                game.move_piece((pos_start), (pos_end));
            }
        }

        let string = game.board_to_string(true);
        println!("{}", string);

        let pos_start = (0, 3);
        let pos_end = (1, 4);

        let valid_moves = game.valid_moves((pos_start));
        //println!("The valid moves are: {:?}", valid_moves);
        for i in &valid_moves {
            let (a, b) = i.last().unwrap();
            //println!("{:?}",b);
            if b == &pos_end {
                println!("The move is valid!");
                game.move_piece((pos_start), (pos_end));
            }
        }

        let string = game.board_to_string(true);
        println!("{}", string);

        let pos_start = (0, 6);
        let pos_end = (0, 1);
        let mut bool = false;

        let valid_moves = game.valid_moves((pos_start));
        //println!("The valid moves are: {:?}", valid_moves);
        for i in &valid_moves {
            let (a, b) = i.last().unwrap();
            //println!("{:?}",b);
            if b == &pos_end {
                game.move_piece(pos_start, pos_end);
                bool = true;
            }
        }

        if bool {
            println!("------------------");
            println!("The move is valid!");
            println!("------------------");
            let string = game.board_to_string(true);
            println!("{}", string);
        } else {
            println!("------------------");
            println!("The move is invalid!");
            println!("------------------");
        }

        let pos_start = (0, 6);
        let pos_end = (0, 5);
        let mut bool = false;

        let valid_moves = game.valid_moves((pos_start));
        //println!("The valid moves are: {:?}", valid_moves);
        for i in &valid_moves {
            let (a, b) = i.last().unwrap();
            //println!("{:?}",b);
            if b == &pos_end {
                game.move_piece(pos_start, pos_end);
                bool = true;
            }
        }
        let string = game.board_to_string(true);
        println!("{}", string);

        let check_vic = game.check_victory();
        println!("Has there been a victory yet?: {:?}", check_vic);
    }
}