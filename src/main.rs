/* use db_helper::initdb;
use model::DBError; */

use surrealdb::opt::auth::Root;
use tokio::net::TcpListener;
/* mod cash;
mod db_helper;
mod model;
mod stock;
mod user; */
use axum::{response::Html, routing::get, Router};

/* #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cashsum {
    pub currency: String,
    pub sum: u64,
    pub owner: Thing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
} */

/* fn create_entries(table: &HashMap<&str, Vec<(&str, &str)>>) -> String {
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
} */

/* fn create_select(table: &Vec<(&str, Vec<&str>)>) -> String {
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
} */

/* fn create_update(table: &HashMap<&str, Vec<(&str, &str)>>) -> String {
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
} */
/*
struct Relate<'a> {
    source: (&'a str, &'a str),
    target: (&'a str, &'a str),
} */
//&[((&str, &str), (&str, &str))]
/* #[allow(unused)]
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
} */

/* impl From<surrealdb::error::Db> for DBError {
    fn from(_value: surrealdb::error::Db) -> Self {
        Self::Sdb
    }
}

impl From<surrealdb::Error> for DBError {
    fn from(_value: surrealdb::Error) -> Self {
        Self::Db(_value)
    }
}
 */
//Result<impl Iterator<Item = Result<Object>>, DBError>
/* #[allow(unused)]
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
} */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_service::db_helper::initdb("e").await?;
    //let db = initdb("e").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;
    let ii = db_service::DB { db: &db };

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

    let routes_all = Router::new().route("/", get(|| async { Html("Hello, World!") }));
    // region:    --- Start Server
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
