use crate::engine::VictoryStatus;
use crate::state::ChessMove;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetHostGames { host: String },
    GetAdmin {},
    GetWins { player: String },
    CheckMatch { host: String, opponent: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateAdmin {
        admin: Option<String>,
    },
    PlayMove {
        host: String,
        opponent: String,
        your_move: ChessMove,
    },
    StartMatch {
        opponent: String,
        first_move: ChessMove,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GameMove {
    Rock,
    Paper,
    Scissors,
    NotPlayed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Game {
    pub host_move: GameMove,
    pub opp_move: GameMove,
    pub result: GameResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ChessMatch {
    pub host: String,
    pub opponent: String,
    pub host_move: GameMove,
    pub opp_move: GameMove,
    pub result: GameResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GameResult {
    Win,
    Loss,
    Tie,
    InProgress,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GameList {
    pub games: Vec<ChessMatch>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GameReponse {
    pub host_move: GameMove,
    pub opp_move: GameMove,
    pub result: GameResult,
}
