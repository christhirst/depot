use std::sync::{Arc, Mutex};

use crate::ctx::Ctx;
use crate::error::Resultc;
use db_service::model::Cash;
use db_service::model::Stock;
use db_service::Record;
use db_service::DB;
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

/* #[derive(Debug, Deserialize, Serialize)]
pub struct Stock {
    //id: Thing,
    pub name: String,
    pub symbol: String,
    pub amount: i64,
    pub price: f64,
    pub owner: Thing, //String,
    pub datebuy: DateTime<Utc>,
} */

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
    pub async fn stock_buy(&self, ctx: Ctx, stock: Stock) -> Resultc<Stock> {
        //let mut store = self.tickets_store.lock().unwrap();
        let stock = Stock {
            owner: Thing::from(("user", ctx.user_id().to_string().as_ref())),
            ..stock
        };

        let _cash = self.db.share_buy(&stock).await?;

        Ok(stock)
    }

    pub async fn stock_sell(&self, ctx: Ctx, stock: Stock) -> Resultc<Stock> {
        //let mut _store = self.tickets_store.lock().unwrap();
        let stock = Stock {
            owner: Thing::from(("user", ctx.user_id().to_string().as_ref())),
            ..stock
        };

        let _cash = self.db.share_sell(&stock).await.unwrap();
        //store.push(Some(ticket.clone()));

        //Ok(ticket)

        Ok(stock)
    }
    pub async fn stock_get(&self, ctx: Ctx, stock: Stock) -> Resultc<Vec<Stock>> {
        //let mut store = self.tickets_store.lock().unwrap();

        let cash = self.db.shares_select("share", &stock).await?;
        Ok(cash)
    }

    pub async fn table_flush(&self, _ctx: Ctx, tb: &str) -> Resultc<Vec<Stock>> {
        let cash = self.db.flushdb(tb).await?;

        Ok(cash)
    }
    pub async fn cash_add(&self, _ctx: Ctx, cash: Cash) -> Resultc<Record> {
        let cash = self.db.cash_entry(&cash).await?;

        Ok(cash)
    }

    pub async fn cash_pull(&self, _ctx: Ctx, _cash: db_service::model::Cash) -> Resultc<Ticket> {
        //let mut _store = self.tickets_store.lock().unwrap();

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
}

// endregion: --- Model Controller
