use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::{any::Any, remote::ws::Client},
    sql::Thing,
    Surreal,
};

pub mod cash;
pub mod db_helper;
pub mod model;
pub mod stock;
pub mod user;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: Option<Thing>,
    pub name: Option<String>,
    pub mail: String,
    pub pw: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Debug, Clone)]
pub struct DB {
    pub db: Surreal<Any>,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
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

        let idx = vec![("symbolIndex", "stock", "symbol")];

        let resp = ii.db_init(&table, &set, &idx).await?;

        let err = resp.check();
        println!("{:?}", err);
        Ok(())
    }

    #[test]
    fn urlsbuilder_test() -> Result<(), Box<dyn std::error::Error>> {
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
        info!("{q:#?}");

        Ok(())
    }
}
