#[allow(unused, dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use surrealdb::dbs::Response;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
mod cash;
mod stock;
mod user;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::{Datetime, Id};
use surrealdb::sql::{Object, Thing, Value};
use surrealdb::Surreal;
use thiserror::Error;

use chrono::{DateTime, Utc};

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: Thing,
    pub name: String,
    pub mail: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cash {
    pub timestamp: String,
    pub currency: String,
    pub amount: String,
    pub owner: String,
}

pub struct Pocket {
    db: Surreal<Any>,
    name: String,
    owner: String,
    all_cash: HashMap<String, Cash>,
    all_stocks: HashMap<String, Stock>,
}

impl Pocket {
    async fn init(mut self) -> Result<(), DBError> {
        let now: DateTime<Utc> = Utc::now();
        println!("{}", now.to_rfc3339());

        // Create database connection
        self.db = surrealdb::engine::any::connect("mem://").await?;
        Ok(())
    }

    //Result<(), DBError>
    async fn new(self, name: String, owner: String) -> Self {
        let now: DateTime<Utc> = Utc::now();
        println!("{}", now.to_rfc3339());

        // Create database connection

        let mut all_cash = HashMap::new();
        all_cash.insert(
            "cash".to_owned(),
            Cash {
                timestamp: todo!(),
                currency: todo!(),
                amount: todo!(),
                owner,
            },
        );
    }

    fn add_cash(&self) -> Option<Cash> {
        todo!()
    }

    fn stock_buy(&self) -> Option<Stock> {
        todo!()
    }

    fn stock_sell(&self) -> Option<Stock> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cashsum {
    pub currency: String,
    pub sum: u64,
    pub owner: Thing,
}

/* #[derive(Debug, Serialize, Deserialize, Clone)]
pub struct cashsum {
    pub currency: String,
    pub sum: u64,
    pub owner: Thing,
} */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ob {
    pub total: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stock {
    #[allow(dead_code)]
    //id: Thing,
    name: String,
    symbol: String,
    amount: i64,
    price: f64,
    owner: String,
    datebuy: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub fn thing_to_string(id: Thing) -> String {
    format!("{}:{}", id.tb, id.id)
}
fn define_table(table: &Vec<&str>) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("DEFINE TABLE {} SCHEMAFULL; ", s);
        q.push_str(&qs)
    }
    q
}

fn define_field(table: &[(&str, &str, &str)]) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("DEFINE FIELD {} ON TABLE {} TYPE {};", s.0, s.1, s.2);
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
        q.push_str("; ");
    }
    //println!("{}", q);
    q
}

fn create_select(table: &Vec<(&str, Vec<&str>)>) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("{} ", s.0);
        q.push_str(&qs);
        for ss in &s.1 {
            let qq = format!("{} ", ss);
            q.push_str(&qq);
        }
    }
    q.push_str("; ");
    q
}

fn create_update(table: &HashMap<&str, Vec<(&str, &str)>>) -> String {
    //UPDATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript'];
    let mut q = String::from("");
    for s in table {
        let qs = format!("UPDATE ONLY {}", s.0);
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
        q.push_str("; ");
    }
    println!("{}", q);
    q
}

struct Relate<'a> {
    source: (&'a str, &'a str),
    target: (&'a str, &'a str),
}
//&[((&str, &str), (&str, &str))]
#[allow(unused)]
fn relate_wrote(table: &[Relate]) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!(
            "RELATE {}:{}->wrote->{}:{} SET time.written = time::now();",
            s.source.0, s.source.1, s.target.0, s.target.1
        );
        q.push_str(&qs)
    }
    q
}

impl From<surrealdb::error::Db> for DBError {
    fn from(_value: surrealdb::error::Db) -> Self {
        Self::Sdb
    }
}

impl From<surrealdb::Error> for DBError {
    fn from(_value: surrealdb::Error) -> Self {
        Self::Db(_value)
    }
}

//Result<impl Iterator<Item = Result<Object>>, DBError>
#[allow(unused)]
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

struct DB<'a> {
    db: &'a Surreal<Client>,
}

// impl of Val
impl<'s> DB<'s> {
    async fn db_init(
        &self,
        table: &Vec<&str>,
        //&[(&str, &str, &str)]
        //&Vec<(&str, &str, &str)>
        fields: &[(&str, &str, &str)],
    ) -> surrealdb::Result<surrealdb::Response> {
        let q = define_table(table);
        let _result = self.db.query(q).await?;
        let q = define_field(fields);
        let result = self.db.query(q).await?;
        Ok(result)
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

fn string_wrap(s: &str) -> String {
    format!("'{}'", s)
}

async fn initdb(s: &str) -> Result<Surreal<Client>, DBError> {
    let db: Surreal<Client>;

    /* if s == "mem" {
        //db = surrealdb::engine::any::connect("mem://").await?;
    } else {
        db = Surreal::new::<Ws>("0.0.0.0:8080").await?;
    } */
    db = Surreal::new::<Ws>("0.0.0.0:8080").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;
    Ok(db)
}

#[tokio::main]
async fn main() -> Result<(), DBError> {
    let now: DateTime<Utc> = Utc::now();
    println!("{}", now.to_rfc3339());

    let db = initdb("e").await?;

    // Create database connection
    //let db = surrealdb::engine::any::connect("mem://").await?;
    // Connect to the server
    //let db = Surreal::new::<Ws>("0.0.0.0:8080").await?;
    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;
    let ii = DB { db: &db };

    //init tables
    let table = vec!["user", "cash", "share", "cashsum"];

    //init fields
    let set = vec![
        //user
        ("name", "user", "string"),
        ("mail", "user", "string"),
        //currency
        ("timestamp", "cash", "datetime"),
        ("currency", "cash", "string"),
        ("amount", "cash", "number"),
        ("owner", "cash", "record(user)"),
        //cashsum
        ("owner", "cashsum", "record(user)"),
        ("currency", "cashsum", "string"),
        ("sum", "cashsum", "number"),
        //share
        ("name", "share", "string"),
        ("owner", "share", "record(user)"),
        ("symbol", "share", "string"),
        ("amount", "share", "number"),
    ];
    let _u = ii.db_init(&table, &set).await?;
    let tb_user1 = String::from("user:testuser1");
    //create user
    let i = Id::from("testuser11");
    let t = Thing {
        id: i,
        tb: "user".to_owned(),
    };

    let user = User {
        id: t,
        name: String::from("'testuser1'"),
        mail: String::from("'testuser1@mail'"),
    };
    /* let uw: cash = ii.cash_add(&tb_user1, "'eur'", "22").await.unwrap();
    println!("{uw:?}");
    let share = Stock {
        name: String::from("'British American Tobacco'"),
        symbol: String::from("'bat'"),
        price: String::from("11"),
        amount: String::from("110000"),
        owner: String::from("record('user:testuser1')"),
        datebuy: String::from("2024-01-01 00:00:00"),
    }; */

    /*
    let share = Stock {
        name: String::from("British American Tobacco"),
        symbol: String::from("bat"),
        price: String::from(""),
        amount: String::from("110000"),
        owner: String::from("user:testuser1"),
        datebuy: String::from("2024-01-01 00:00:00"),
    };

    let uu = ii.stock_sell(&share);

    let set1: Vec<(&str, &str)> = vec![("currency", "'eur'"), ("amount", "100000.0")];
    let set2: Vec<(&str, &str)> = vec![("mail", "'user1@mail.com'"), ("name", "'testuser1'")];
    let mut rpg_party = HashMap::new();
    rpg_party.insert("cash", set1);
    rpg_party.insert("user:testuser1", set2);

    println!("{:?}", "&2222");
    println!("{:?}", create_entries(&rpg_party));
    let mut result = db.query(create_entries(&rpg_party)).await?;

    let oo = ii.stock_sell(&share).await?;

    println!("{:?}", "&3333");
    let r: Option<Record> = result.take(0)?;
    println!("{:?}", "&3333");
    println!("{:?}", r.unwrap()); */

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn db_test() {
        let u = initdb("mem").await.is_ok();

        assert!(u);
    }

    #[tokio::test]
    async fn create_table() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };
        let table = vec!["user"];

        //init fields
        let set = vec![
            //user
            ("name", "user", "string"),
            ("mail", "user", "string"),
        ];

        let resp = ii.db_init(&table, &set).await?;

        let err = resp.check();
        println!("{:?}", err);
        Ok(())
    }

    #[test]
    fn dele_entry() {
        let i = vec!["symbol"];
        let ii = vec!["shares"];

        //let cond = stock.symbol.to_owned();
        let cond = format!("{} = {}", "symbol", "sym");

        let iii = vec![cond.as_str()];

        let set2: Vec<(&str, Vec<&str>)> = vec![("SELECT", i), ("FROM", ii), ("WHERE", iii)];
        let q = create_select(&set2);
        println!("{q:?}");
        assert_eq!(q, "SELECT symbol FROM shares WHERE symbol = sym ; ")
    }

    /* #[tokio::test]
        async fn search_entry() -> Result<(), DBError> {
           let db = initdb("mem").await?;
            let qqq = DB { db: &db };

            let select = vec!["*"];
            let from = vec!["user"];
            let cond = format!("{} = {}", "mail", "'testuser1@mail'");
            let wher = vec![cond.as_str()];

            let set2: Vec<(&str, Vec<&str>)> =
                vec![("SELECT", select), ("FROM", from), ("WHERE", wher)];

            let q = create_select(&set2);

            let share = Stock {
                name: String::from("'British American Tobacco'"),
                symbol: String::from("'bat'"),
                price: String::from("11"),
                amount: String::from("110000"),
                owner: String::from("record('user:testuser1')"),
                datebuy: String::from("2024-01-01 00:00:00"),
            };
            let oo = qqq.stock_sell(&share).await?;

            println!("{oo:?}");
            Ok(())
        }
    */
    #[test]
    fn delete_entr() -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    #[test]
    fn get_sum() -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    #[test]
    fn urlsbuilder_test() -> Result<(), Box<dyn std::error::Error>> {
        let table = vec!["user", "cash", "share"];

        //init fields
        let fields = vec![
            //user
            ("name", "user", "string"),
        ];
        let q = define_table(&table);
        println!("{q:?}");
        let q = define_field(&fields);
        println!("{q:#?}");

        let mut rpg_party = HashMap::new();
        let set1: Vec<(&str, &str)> = vec![("currency", "'eur'"), ("amount", "100000.0")];
        let set2: Vec<(&str, &str)> = vec![("mail", "'user1@mail.com'")];

        rpg_party.insert("cash", set1);
        rpg_party.insert("user:testuser1", set2);
        let q = create_entries(&rpg_party);
        println!("{q:#?}");

        let i = vec!["*"];
        let ii = vec!["cash"];

        //let cond = stock.symbol.to_owned();
        let cond = format!("{} = {}", "currency", "'eur'");

        let iii = vec![cond.as_str()];

        let set2: Vec<(&str, Vec<&str>)> = vec![("SELECT", i), ("FROM", ii), ("WHERE", iii)];

        let q = create_select(&set2);
        println!("{q:#?}");

        /* let filename1 = "Config.toml";
        let conf = load_or_initialize(filename1).unwrap();
        let urlresult = format!(
            "{}/{}+eq+{}",
            conf.baseurl, conf.urlfilter[0].0, conf.urlfilter[0].1[0]
        );

        let n = httprequests::urlsbuilder(&conf.baseurl, &conf.urlfilter);
        println!("{n:?}");
        println!("--------------"); */

        //assert_eq!(urlresult, n);
        Ok(())
    }
}
