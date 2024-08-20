use std::sync::{Arc, Mutex};

use crate::ctx::Ctx;
use crate::error::Resultc;
use db_service::model::Cash;
use db_service::model::Stock;
use db_service::model::StockEntry;
use db_service::stock::PriceSum;
use db_service::stock::Sum;
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
    pub async fn stock_add(&self, ctx: Ctx, stockentry: StockEntry) -> Resultc<StockEntry> {
        //let mut _store = self.tickets_store.lock().unwrap();
        let stockentry = self.db.stock_add(&stockentry).await?;
        Ok(stockentry)
    }
    pub async fn stock_list(&self, ctx: Ctx, stocks: String) -> Resultc<Vec<StockEntry>> {
        //let mut _store = self.tickets_store.lock().unwrap();
        let stockentry = self.db.stock_list(&stocks).await?;
        Ok(stockentry)
    }

    pub async fn stock_del(&self, ctx: Ctx, stock: Stock) -> Resultc<Stock> {
        //let mut _store = self.tickets_store.lock().unwrap();
        let stock = Stock {
            owner: Thing::from(("user", ctx.user_id().to_string().as_ref())),
            ..stock
        };
        let _cash = self.db.share_sell(&stock).await.unwrap();
        Ok(stock)
    }

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
        Ok(stock)
    }

    pub async fn stock_get(&self, ctx: Ctx, stock: Stock) -> Resultc<Vec<Stock>> {
        //let mut store = self.tickets_store.lock().unwrap();
        let cash = self.db.shares_select("share", &stock).await?;
        Ok(cash)
    }
    pub async fn stock_amount_by_symbol(&self, ctx: Ctx, symbol: &str) -> Resultc<Sum> {
        //let mut store = self.tickets_store.lock().unwrap();
        let cash = self.db.share_sum(symbol).await?;
        Ok(cash)
    }

    pub async fn stock_worth_by_symbol(&self, ctx: Ctx, symbol: &str) -> Resultc<PriceSum> {
        //let mut store = self.tickets_store.lock().unwrap();
        let cash = self.db.share_price_sum(symbol).await?;
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

    pub async fn cash_pull(&self, _ctx: Ctx, cash: Cash) -> Resultc<Record> {
        if cash.amount < 0.0 {
            let rec = self.db.cash_entry(&cash).await?;
            Ok(rec)
        } else {
            let cash = Cash {
                amount: -cash.amount,
                ..cash
            };
            let rec = self.db.cash_entry(&cash).await?;
            Ok(rec)
        }
    }

    pub async fn cash_sum_by_currency(&self, ctx: Ctx, currency: &str) -> Resultc<Vec<Ticket>> {
        self.db.cash_sum(
            &Thing::from(("user", ctx.user_id().to_string().as_ref())),
            currency,
        );
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn entry_del(&self, _ctx: Ctx, cash: Thing) -> Resultc<StockEntry> {
        let stockentry = self.db.entry_del(&cash).await?;
        Ok(stockentry)
    }
}

// endregion: --- Model Controller
