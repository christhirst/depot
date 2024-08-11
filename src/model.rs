use std::sync::{Arc, Mutex};

use crate::error::Resultc;
use crate::{ctx::Ctx, error};
use chrono::{DateTime, Utc};
use db_service::model::Cash;
use db_service::DB;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
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
    pub datebuy: DateTime<Utc>,
}

// region:    --- Ticket Types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
// endregion: --- Ticket Types
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
    db: DB,
}

// Constructor
impl ModelController {
    pub async fn new() -> Resultc<Self> {
        let db = db_service::db_helper::initdb("e").await.unwrap();
        Ok(Self {
            tickets_store: Arc::default(),
            db: DB { db },
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn cash_add(&self, ctx: Ctx, cash: db_service::model::Cash) -> Resultc<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        /* let cash = self.db.cash_add(cash).await?;
        store.push(Some(ticket.clone()));

        Ok(ticket) */
        todo!()
    }

    pub async fn get_cash_sum(
        &self,
        // _ctx: Ctx,
        owner: &Thing,
        currency: &str,
    ) -> Resultc<Vec<Ticket>> {
        self.db.cash_sum(owner, currency);
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Resultc<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(error::Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion: --- Model Controller
