use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

//use tonic_reflection::server::Error as Grpc_Error;

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum DBError {
    #[error("Database error")]
    Sdb,
    #[error("{0}")]
    Db(#[from] surrealdb::Error),
    #[error("eee")]
    OO(),
    #[error("Cash error")]
    CashErr(),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cash {
    pub currency: String,
    pub amount: f64,
    pub owner: Thing,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stock {
    //id: Thing,
    pub name: String,
    pub symbol: String,
    pub amount: i64,
    pub price: f64,
    pub owner: Thing, //String,
    pub datebuy: DateTime<Utc>,
}

#[allow(unused)]
enum Typeinto {
    Int(i32),
    Float(f64),
    Text(String),
}

impl fmt::Display for Typeinto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Typeinto::Int(a) => write!(f, "{}", a),
            Typeinto::Float(a) => write!(f, "{}", a),
            Typeinto::Text(a) => write!(f, "{}", a),
        }
    }
}
