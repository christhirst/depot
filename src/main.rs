use std::collections::{hash_map, HashMap};

#[allow(unused, dead_code)]
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::{self, Any};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

/*
#[derive(Debug, Clone)]
pub enum MyError {
    // Define different variants of your error type
    CacheError,
    OtherError(String),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stock {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cash {
    pub euro: i64,
}

#[allow(dead_code)]
pub struct Pocket {
    all_cash: RwLock<Option<Vec<Cash>>>,
    all_stocks: RwLock<Option<Vec<Stock>>>,
}
 #[allow(dead_code)]
impl Pocket {
    fn new() -> Self {
        Pocket {
            all_cash: RwLock::new(None),
            all_stocks: RwLock::new(None),
        }
    }
    async fn all_stocks(&self) -> Option<Vec<Stock>> {
        let lock: tokio::sync::RwLockReadGuard<'_, Option<Vec<Stock>>> =
            self.all_stocks.read().await;
        lock.clone()
    }
    async fn refresh_cash(&self, stocks: Vec<Stock>) {
        let mut lock = self.all_stocks.write().await;
        *lock = Some(stocks);
    }
    async fn refresh_stocks(&self, cash: Vec<Cash>) {
        let mut lock = self.all_cash.write().await;
        *lock = Some(cash);
    }
    async fn invalidate(&self) {
        let mut lock = self.all_stocks.write().await;
        *lock = None;
    }

    fn add_cash() {}
    fn remove_cash() {}
    fn add_stock() {}
    fn sell_stock() {}
} */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cash {
    pub euro: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pocket {
    all_cash: Cash,
    all_stocks: HashMap<String, Stock>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stock {
    #[allow(dead_code)]
    name: String,
    symbol: String,
    amount: i64,
}
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

struct DB<'a> {
    db: &'a Surreal<Any>,
}

// impl of Val
impl<'s> DB<'s> {
    async fn flushdb(self, table: &str) -> surrealdb::Result<Vec<Record>> {
        let rec: Vec<Record> = self.db.delete(table).await?;
        Ok(rec)
    }
    async fn buy<'q>(self, stock: HashMap<String, Stock>, price: i64) -> surrealdb::Result<()> {
        let pocket: Option<Pocket> = self.db.select(("test", "test")).await?;
        let mut pocke = pocket.unwrap();
        pocke.all_cash.euro -= stock[""].amount * price;
        //let sto = pocke.all_stocks.get(".).unwrap();

        let people: Option<Pocket> = self
            .db
            .update(("stock", "ii"))
            .merge(Pocket {
                all_cash: Cash {
                    euro: pocke.all_cash.euro,
                },
                all_stocks: stock,
            })
            .await?;
        Ok(())
    }

    async fn sell(db: Surreal<Any>, table: &str) -> surrealdb::Result<()> {
        let people: Vec<Record> = db.delete(table).await?;
        Ok(())
    }
}

/* #[derive(Debug, Deserialize, Serialize)]
struct Stocks {
    #[allow(dead_code)]
    name:  String,
    amount: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pockets {
    all_cash: Cash,
    all_stocks:  Vec<Stocks>,
}
 */

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = surrealdb::engine::any::connect("mem://").await?;
    //let people: Vec<Record> = db.delete("test").await?;
    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    let ii = DB { db: &db };

    let mut m = HashMap::new();
    m.insert(
        "k".to_owned(),
        Stock {
            name: "teste".to_owned(),
            symbol: "teste".to_owned(),
            amount: 2,
        },
    );

    // Create a new person with a random id
    /* let created: Option<Stock> = db
    .create(("stock", "ii"))
    .content(Pocket {
        all_cash: Cash { euro: 2 },
        all_stocks: m,
    })
    .await?; */

    //dbg!(created);

    let mut m = HashMap::new();
    m.insert(
        "k".to_owned(),
        Stock {
            name: "".to_owned(),
            symbol: "".to_owned(),
            amount: 2,
        },
    );

    ii.buy(m, 22);

    // Update a person record with a specific id
    /* let updated: Option<Record> = db
    .update(("stock", "ii"))
    .merge(Pocket {
        all_cash: Cash { euro: 1 },
        all_stocks: m,
    })
    .await?; */
    //dbg!(updated);

    // Select all people records
    println!("{}", "qqqq");
    let people: Vec<Pocket> = db.select("stock").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    //dbg!(groups);

    Ok(())
}
