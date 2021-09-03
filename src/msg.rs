use crate::state::ChessMove;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAdmin {},
    CheckMatch { host: String, opponent: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
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
