use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use thiserror::Error;
//use tonic_reflection::server::Error as Grpc_Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database error")]
    Sdb,
    #[error("Database error")]
    Db(surrealdb::Error),
    #[error("eee")]
    OO(),
    #[error("Cash error")]
    CashErr(),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stock {
    //id: Thing,
    pub name: String,
    pub symbol: String,
    pub amount: i64,
    pub price: f64,
    pub owner: Thing, //String,
    pub datebuy: String,
}
