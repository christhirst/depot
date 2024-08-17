use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::engine::any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

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
    #[error("{0}")]
    SDA(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cash {
    pub id: Option<Thing>,
    pub currency: String,
    pub amount: f64,
    pub owner: Thing,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stock {
    pub id: Option<Thing>,
    pub name: String,
    pub symbol: String,
    pub amount: i64,
    pub price: f64,
    pub owner: Thing, //String,
    pub datebuy: DateTime<Utc>,
}

impl Default for Stock {
    fn default() -> Stock {
        Stock {
            id: None,
            ..Default::default()
        }
    }
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
