use std::collections::HashMap;
use std::fmt;

#[allow(unused, dead_code)]
use serde::{Deserialize, Serialize};
use surrealdb::dbs::Response;
use surrealdb::engine::any::Any;

use surrealdb::sql::{Object, Thing, Value};
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

fn define_table(table: Vec<&str>) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("DEFINE TABLE {} SCHEMAFULL;\n ", s);
        q.push_str(&qs)
    }
    q
}

fn define_field(table: &Vec<(&str, &str, &str)>) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("DEFINE FIELD {} ON TABLE {} TYPE {};\n", s.0, s.1, s.2);
        q.push_str(&qs)
    }
    q
}

fn create_entries(table: &HashMap<&str, Vec<(&str, &str)>>) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("CREATE {}", s.0);
        q.push_str(&qs);

        let mut i = 0;
        for ss in s.1 {
            i += 1;
            if i == 1 {
                q.push_str(" SET");
            }
            let qs = format!(" {} = {}", ss.0, ss.1);
            q.push_str(&qs);
            if s.1.len() != i {
                q.push(',')
            }
        }
        q.push_str("; \n ");
    }
    println!("test {}", q);
    q
}

fn relate_wrote(table: &Vec<((&str, &str), (&str, &str))>) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!(
            "RELATE {}:{}->wrote->{}:{} SET time.written = time::now();\n",
            s.0 .0, s.0 .1, s.1 .0, s.1 .1
        );
        q.push_str(&qs)
    }
    q
}

#[derive(Debug)]
pub enum DBError {
    Sdb,
}

impl From<surrealdb::error::Db> for DBError {
    fn from(value: surrealdb::error::Db) -> Self {
        Self::Sdb
    }
}

//Result<impl Iterator<Item = Result<Object>>, DBError>
fn into_iter_objects(
    ress: Vec<Response>,
) -> Result<impl Iterator<Item = Result<Object, DBError>>, DBError> {
    let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;
    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(DBError::Sdb),
            });
            Ok(it)
        }
        _ => Err(DBError::Sdb),
    }
}

// impl of Val
impl<'s> DB<'s> {
    async fn db_init(
        &self,
        table: Vec<&str>,
        fields: &Vec<(&str, &str, &str)>,
    ) -> surrealdb::Result<surrealdb::Response> {
        let q = define_table(table);
        let mut result = self.db.query(q).await?;
        let q = define_field(fields);
        let mut result = self.db.query(q).await?;
        Ok(result)
    }
    async fn user_add(&self, user: &str, mail: &str) -> surrealdb::Result<()> {
        //create_entries()

        Ok(())
    }
    async fn user_del(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
    async fn user_get(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }

    async fn cash_add(&self, table: &str) -> surrealdb::Result<()> {
        let set1: Vec<(&str, &str)> = vec![("currency", "'eur'"), ("amount", "100000.0")];
        let mut rpg_party = HashMap::new();
        rpg_party.insert("cash", set1);
        create_entries(&rpg_party);

        Ok(())
    }
    async fn cash_del(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
    async fn cash_get(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }

    async fn share_buy(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
    async fn share_sell(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
    async fn share_get(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }

    async fn flushdb(&self, table: &str) -> surrealdb::Result<Vec<Record>> {
        let rec: Vec<Record> = self.db.delete(table).await?;
        Ok(rec)
    }
    async fn buy<'q>(&self, stock: HashMap<String, Stock>, price: i64) -> surrealdb::Result<()> {
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

    async fn cashs_add(self, cash: &Cash) -> surrealdb::Result<()> {
        /* TODO pocket
         ** create user
         ** create pocket linked to user
         ** create shares linked to pocket
         */

        /* let q = format!(
            "
            DEFINE TABLE users SCHEMAFULL;
            CREATE users:test1 SET mail = 'user1@mail.com';

            DEFINE TABLE shares SCHEMAFULL;
            DEFINE FIELD symbol ON TABLE shares TYPE string;
            DEFINE FIELD amount ON TABLE shares TYPE number;


            DEFINE TABLE cash SCHEMAFULL;
            DEFINE FIELD amount ON TABLE cash TYPE number;
            DEFINE FIELD currency ON TABLE cash TYPE string;
            CREATE cash:1 SET currency = 'eur', amount = 110000, users:test1;
            CREATE cash:2 SET currency = 'eur', amount = 10000, users:test1;


            RELATE users:test1->wrote->cash:1 SET time.written = time::now();
            SELECT * FROM cash:1;",
        ); */

        //RELATE users:test1->wrote->cash:1 SET time.written = time::now();

        //let mut result = self.db.query(q).await?;

        //let r: Option<Record> = result.take(7)?;
        //println!("{:?}", r.unwrap());

        println!("{}", "4");
        Ok(())
    }
}

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

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = surrealdb::engine::any::connect("mem://").await?;
    db.use_ns("test").use_db("test").await?;
    let ii = DB { db: &db };

    //init tables
    let table = vec!["user", "cash", "share"];

    //init fields
    let set = vec![
        ("mail", "user", "string"),
        ("currency", "cash", "string"),
        ("amount", "cash", "number"),
        ("symbol", "share", "string"),
        ("amount", "share", "number"),
    ];
    ii.db_init(table, &set);
    ii.cash_add("table");

    let set1: Vec<(&str, &str)> = vec![("currency", "'eur'"), ("amount", "100000.0")];
    let set2: Vec<(&str, &str)> = vec![("mail", "'user1@mail.com'")];
    let mut rpg_party = HashMap::new();
    rpg_party.insert("cash", set1);
    rpg_party.insert("user", set2);

    println!("{:?}", "&2222");
    println!("{:?}", create_entries(&rpg_party));
    let mut result = db.query(create_entries(&rpg_party)).await?;

    println!("{:?}", "&3333");
    let r: Option<Record> = result.take(0)?;
    println!("{:?}", r.unwrap());

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

    // Update a person record with a specific id
    /* let updated: Option<Record> = db
    .update(("stock", "ii"))
    .merge(Pocket {
        all_cash: Cash { euro: 1 },
        all_stocks: m,
    })
    .await?; */

    //dbg!(groups);

    Ok(())
}
