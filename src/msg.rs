use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetHostGames {host: String},
    GetGame { host: String, opponent: String},
    GetAdmin {} ,
    GetWins {player: String},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StartGame { opponent: String, start_move: GameMove},
    UpdateAdmin {admin: Option<String>},
    AddHook { addr: String },
    RemoveHook { addr: String },
    Respond {host:String, opp_move: GameMove},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameMove {
    Rock,
    Paper,
    Scissors,
    NotPlayed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
    pub host_move: GameMove,
    pub opp_move: GameMove,
    pub result: GameResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ChessMatch {
    pub host: String,
    pub opponent: String,
    pub host_move: GameMove,
    pub opp_move: GameMove,
    pub result: GameResult,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameResult {
    Win,
    Loss,
    Tie,
    InProgress,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct GameList {
    pub games: Vec<ChessMatch>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameReponse {
    pub host_move: GameMove,
    pub opp_move: GameMove,
    pub result: GameResult,
}