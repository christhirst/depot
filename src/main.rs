use std::collections::{hash_map, HashMap};

#[allow(unused, dead_code)]
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::{self, Any};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
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
    pub currency: String,
    pub amount: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pocket {
    name: String,
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
    async fn user_add(self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
    async fn user_del(self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
    async fn user_get(self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }

    async fn flushdb(self, table: &str) -> surrealdb::Result<Vec<Record>> {
        let rec: Vec<Record> = self.db.delete(table).await?;
        Ok(rec)
    }
    async fn buy<'q>(self, stock: HashMap<String, Stock>, price: i64) -> surrealdb::Result<()> {
        // Run some queries
        let query = "
CREATE person;
SELECT * FROM type::table($table);
";
        //add amount, add, transaction,
        self.db.query(query).bind(("table", "person")).await?;

        let pocket: Option<Pocket> = self.db.select(("test", "test")).await?;
        let mut pocke = pocket.unwrap();

        //let sto = pocke.all_stocks.get(".).unwrap();

        Ok(())
    }

    async fn stock_sell(self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }

    async fn cash_add(self, cash: &Cash) -> surrealdb::Result<()> {
        println!("{}", "3");
        let query = "
        DEFINE TABLE cash SCHEMALESS;
        DEFINE FIELD amount ON TABLE cash TYPE number;
        DEFINE FIELD currency ON TABLE cash TYPE string;
        CREATE cash SET currency = 'eur', amount = 10000;
        SELECT * FROM cash;
        ";
        let q = format!(
            "
            CREATE users:test1 SET mail = 'user1@mail.com';
            DEFINE TABLE cash SCHEMAFULL;
            DEFINE FIELD amount ON TABLE cash TYPE number;
            DEFINE FIELD currency ON TABLE cash TYPE string;
            CREATE cash:1 SET currency = 'eur', amount = 110000;
            CREATE cash:2 SET currency = 'eur', amount = 10000;
            RELATE users:test1->wrote->cash:1 SET time.written = time::now();
            SELECT * FROM cash:1;",
        );
        //RELATE users:test1->wrote->cash:1 SET time.written = time::now();

        let mut result = self.db.query(q).await?;

        let r: Option<Record> = result.take(7)?;
        println!("{:?}", r.unwrap());

        println!("{}", "4");
        Ok(())
    }
    async fn cash_get(self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = surrealdb::engine::any::connect("mem://").await?;
    //let people: Vec<Record> = db.delete("test").await?;
    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    let ii = DB { db: &db };

    //let mut m: HashMap<String, Stock> = HashMap::new();
    /*  let s = Stock {
              name: "teste".to_owned(),
              symbol: "teste".to_owned(),
              amount: 2,
          };

       m.insert("stock".to_owned(), s);

    let p = Pocket {
        name: "pocketname".to_owned(),
        all_cash: Cash { euro: 0 },
        all_stocks: m,
    };  */

    // Create a new person with a random id
    //let created: Option<Stock> = db.create(("stock", "iiq")).content(p).await?;

    //dbg!(created);
    let cash = Cash {
        currency: "".to_owned(),
        amount: 10000,
    };
    println!("{:?}", &cash);
    let result = ii.cash_add(&cash).await?;
    //let created: Option<Cash> = result.take(0)?;
    println!("{:?}", "&cash");
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
    /*   println!("{}", "qqqq");
    let people: Vec<Stock> = db.select("stock").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?; */
    //dbg!(groups);

    Ok(())
}
