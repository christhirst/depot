use std::collections::HashMap;
use std::fmt;
use std::path::Path;

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
pub struct User {
    pub name: String,
    pub mail: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cash {
    pub currency: String,
    pub amount: String,
    pub owner: String,
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
    amount: String,
    price: String,
    owner: String,
    datebuy: String,
}
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

struct DB<'a> {
    db: &'a Surreal<Any>,
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

fn create_select(table: &Vec<(&str, &Vec<&str>)>) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("{} ", s.0);
        q.push_str(&qs);
        for ss in s.1 {
            let qq = format!("{} ", ss);
            q.push_str(&qq);
        }
    }
    q.push_str("; ");
    q
}

fn update_entry(table: &HashMap<&str, Vec<(&str, &str)>>) -> String {
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

#[allow(unused)]
fn relate_wrote(table: &[((&str, &str), (&str, &str))]) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!(
            "RELATE {}:{}->wrote->{}:{} SET time.written = time::now();",
            s.0 .0, s.0 .1, s.1 .0, s.1 .1
        );
        q.push_str(&qs)
    }
    q
}

#[derive(Debug)]
pub enum DBError {
    Sdb,
    Db(surrealdb::Error),
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

    #[allow(unused)]
    async fn user_add(&self, u: &str, user: &User) -> Result<User, DBError> {
        let set1: Vec<(&str, &str)> = vec![("name", &user.name), ("mail", &user.mail)];
        let mut rpg_party = HashMap::new();
        rpg_party.insert(u, set1);

        let query = create_entries(&rpg_party);

        let mut result = self.db.query(query).await?;
        let pp: Option<User> = result.take(0).unwrap();
        /*  println!("Failed --------!!--------- {:?}", pp);
        let mut result = self
            .db
            .query("SELECT * FROM user WHERE user:testuser1;")
            .await?; */
        /* if let Err(e) = result.take::<Option<User>>(0) {
            println!("Failed to make a user: {e:#?}");
        } */
        /* let pp: Option<User> = result.take(0).unwrap();
        println!("Failed -----------------: {:?}", pp); */
        pp.ok_or(DBError::Sdb)
    }

    #[allow(unused)]
    async fn user_del(&self, table: &str) -> surrealdb::Result<()> {
        //DELETE person:Tobie@web.de;
        Ok(())
    }

    #[allow(unused)]
    async fn user_get(&self, table: &str) -> surrealdb::Result<()> {
        //DELETE person:Tobie@web.de;
        Ok(())
    }

    async fn cash_add(&self, cash: &Cash) -> Result<Cash, DBError> {
        //CREATE cash SET currency = 'eur', amount = 110000, owner = users:Tobie@web.de;
        let set1: Vec<(&str, &str)> = vec![
            ("currency", &cash.currency),
            ("amount", &cash.amount),
            ("owner", &cash.owner),
        ];
        let mut rpg_party = HashMap::new();
        rpg_party.insert("cash", set1);
        let query = create_entries(&rpg_party);
        let mut result = self.db.query(query).await?;
        let pp: Option<Cash> = result.take(0).unwrap();
        pp.ok_or(DBError::Sdb)
    }

    #[allow(unused)]
    async fn cash_del(&self, table: &str) -> surrealdb::Result<()> {
        //SELECT * FROM cash WHERE owner='users:Tobie@web.de' AND currency='eur';
        //DELETE cash:id;
        Ok(())
    }

    #[allow(unused)]
    async fn cash_get(&self, table: &str) -> surrealdb::Result<()> {
        //SELECT * FROM cash WHERE owner='users:Tobie@web.de' AND currency='eur';

        Ok(())
    }
    #[allow(unused)]
    async fn share_buy(&self, table: &str) -> surrealdb::Result<()> {
        //CREATE share:2 SET sym = 'aurub', amount = 10000, owner = users:test1;

        Ok(())
    }
    #[allow(unused)]
    async fn share_sell(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
    #[allow(unused)]
    async fn share_get(&self, table: &str) -> surrealdb::Result<()> {
        Ok(())
    }
    #[allow(unused)]
    async fn flushdb(&self, table: &str) -> surrealdb::Result<Vec<Record>> {
        let rec: Vec<Record> = self.db.delete(table).await?;
        Ok(rec)
    }
    #[allow(unused)]
    async fn buy<'q>(&self, stock: &Stock) -> surrealdb::Result<()> {
        println!("{}", "++++++++++++++++++++++");
        //CREATE shares SET name = 'British American Tobacco', symbol = 'bat', amount = 110000, owner = users:Tobie@web.de;

        let set1: Vec<(&str, &str)> = vec![
            ("name", &stock.name),
            ("symbol", &stock.symbol),
            ("amount", &stock.amount),
            ("owner", &stock.owner),
        ];

        let mut rpg_party: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
        rpg_party.insert("shares", set1);

        create_entries(&rpg_party);

        Ok(())
    }
    #[allow(unused)]
    async fn stock_sell(&self, stock: &Stock) -> surrealdb::Result<()> {
        //SELECT user FROM events WHERE type = 'activity' GROUP ALL;
        let i = &vec!["symbol"];
        let ii = &vec!["shares"];

        //let cond = stock.symbol.to_owned();
        let cond = format!("{} = {}", "symbol", stock.symbol.to_owned());

        let iii = &vec![cond.as_str()];

        let set2: Vec<(&str, &Vec<&str>)> = vec![("SELECT", i), ("FROM", ii), ("WHERE", iii)];
        let oo = create_select(&set2);

        let mut result = self.db.query("SELECT * FROM shares;").await?;
        panic!("{:?}", result);

        /*
        get stock entry, from date, calculate difference amount
        --> add cash to pocket
        -- Update just a single record
        -- Using the ONLY keyword, just an object for the record in question will be returned.
        -- This, instead of an array with a single object.
        UPDATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript']; */
        //SELECT * FROM person WHERE email='tobie@surrealdb.com' AND company='SurrealDB';
        //select share by price

        let set1: Vec<(&str, &str)> = vec![
            ("name", &stock.name),
            ("symbol", &stock.symbol),
            ("amount", &stock.amount),
            ("owner", &stock.owner),
        ];

        let mut rpg_party = HashMap::new();

        rpg_party.insert("share", set1);
        update_entry(&rpg_party);

        Ok(())
    }
    #[allow(unused)]
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


            CREATE share:2 SET sym = 'aurub', amount = 10000, users:test1;



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
async fn main() -> Result<(), DBError> {
    // Create database connection
    let db = surrealdb::engine::any::connect("mem://").await?;
    db.use_ns("test").use_db("test").await?;
    let ii = DB { db: &db };

    //init tables
    let table = vec!["user", "cash", "share"];

    //init fields
    let set = vec![
        //user
        ("name", "user", "string"),
        ("mail", "user", "string"),
        //currency
        ("currency", "cash", "string"),
        ("owner", "cash", "record(user)"),
        ("amount", "cash", "number"),
        //share
        ("name", "share", "string"),
        ("owner", "share", "record(user)"),
        ("symbol", "share", "string"),
        ("amount", "share", "number"),
    ];
    let u = ii.db_init(&table, &set).await?;

    //create user
    let user = User {
        name: String::from("'testuser1'"),
        mail: String::from("'testuser1@mail'"),
    };
    let uu = ii.user_add("user:testuser1", &user).await?;
    println!("{uu:?}");

    let cash = Cash {
        currency: String::from("eur"),
        amount: String::from("22"),
        owner: String::from("record(user:testuser1)"),
    };
    let uw = ii.cash_add(&cash).await?;

    let share = Stock {
        name: String::from("British American Tobacco"),
        symbol: String::from("bat"),
        price: String::from(""),
        amount: String::from("110000"),
        owner: String::from("record(user:testuser1)"),
        datebuy: String::from("2024-01-01 00:00:00"),
    };

    let uu = ii.buy(&share).await?;

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

#[cfg(test)]
mod tests {
    use super::*;

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

        let i = &vec!["*"];
        let ii = &vec!["cash"];

        //let cond = stock.symbol.to_owned();
        let cond = format!("{} = {}", "currency", "'eur'");

        let iii = &vec![cond.as_str()];

        let set2: Vec<(&str, &Vec<&str>)> = vec![("SELECT", i), ("FROM", ii), ("WHERE", iii)];

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
