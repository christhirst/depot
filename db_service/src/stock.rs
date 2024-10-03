use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use crate::{
    model::{Cash, DBError, Stock, StockEntry},
    Record, User, DB,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sum {
    #[allow(dead_code)]
    pub sum: i64,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceSum {
    #[allow(dead_code)]
    pub sum: f64,
    pub symbol: String,
}

/* #[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashSum {
    #[allow(dead_code)]
    sum: f64,
    symbol: String,
} */

#[derive(Debug, Serialize, Deserialize, Clone)]
struct shares_price {
    #[allow(dead_code)]
    amount: i64,
    price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ID {
    #[allow(dead_code)]
    id: Thing,
}

impl DB {
    #[allow(unused)]
    pub async fn flushdb(&self, table: &str) -> Result<Vec<Stock>, DBError> {
        let rec: Vec<Stock> = self.db.delete(table).await?;

        Ok(rec)
    }

    #[allow(unused)]
    pub async fn stock_add(&self, se: &StockEntry) -> Result<StockEntry, DBError> {
        let query = format!(
            "CREATE stock SET name = '{}', wkn = '{}', isin = '{}', symbol  = '{}', country = '{}';",
            se.name,
            se.wkn.clone().unwrap_or_default(),
            se.isin,
            se.symbol.clone().unwrap_or_default(),
            se.country
        );
        let mut resp = self.db.query(query).await?;
        let created: Option<StockEntry> = resp.take(0)?;
        created.ok_or(DBError::SerializeErr("Error parsing response".to_string()))
    }

    #[allow(unused)]
    pub async fn stock_list(&self, stock: &str) -> Result<Vec<StockEntry>, DBError> {
        let rec: Vec<StockEntry> = self.db.select(stock).await?;

        Ok(rec)
    }

    #[allow(unused)]
    pub async fn entry_del(&self, table: (String, String)) -> Result<StockEntry, DBError> {
        //TODO
        let rec: Option<StockEntry> = self.db.delete(table).await.unwrap();

        rec.ok_or(DBError::Sdb)
    }

    #[allow(unused)]
    pub async fn share_buy(&self, stock: &Stock) -> Result<Record, DBError> {
        //get Cash state with margin
        let cash = self.cash_sum(&stock.owner, "eur").await?;
        //check if cash is enough with 5% margin
        if cash.sum > (stock.amount as f64 * stock.price) * 1.05 {
            //CREATE share:2 SET sym = 'aurub', amount = 10000, owner = users:test1;
            let query = format!("CREATE share SET stock = {}, name = '{}', symbol = '{}', amount = {},price = {}, owner = {}, datebuy = <datetime>'{}';",     
                    stock.stock,
                    stock.name,
                    stock.symbol,
                    stock.amount,
                    stock.price,
                    stock.owner,
                    stock.datebuy,
                );
            let mut resp = self.db.query(query).await?;
            let created: Option<Record> = resp.take(0)?;
            created.ok_or(DBError::Sdb)
        } else {
            Err(DBError::CashErr())
        }
    }
    pub async fn share_sum(&self, stock: &str) -> Result<Sum, DBError> {
        //SELECT math::sum(amount) AS sum,symbol FROM share WHERE symbol = 'bat' GROUP BY symbol;
        let query = format!(
            "SELECT math::sum(amount) AS sum, symbol FROM share WHERE symbol = '{}' GROUP BY symbol;",
            stock,
        );

        let mut result = self.db.query(query).await?;

        let shares: Option<Sum> = result.take(0)?;
        shares.ok_or(DBError::Sdb)
    }

    pub async fn share_price_sum(&self, stock: &str) -> Result<PriceSum, DBError> {
        //SELECT math::sum(amount) AS sum,symbol FROM share WHERE symbol = 'bat' GROUP BY symbol;
        let query = format!(
            "SELECT math::sum(amount * price) AS sum, symbol FROM share WHERE symbol = '{}' GROUP BY symbol;",
            stock,
        );

        let mut result = self.db.query(query).await?;

        let shares: Option<PriceSum> = result.take(0)?;
        shares.ok_or(DBError::Sdb)
    }

    #[allow(unused)]
    pub async fn share_sell(&self, stock: &Stock) -> Result<Record, DBError> {
        let amount = self.share_sum(&stock.symbol).await?;
        if (amount.sum + stock.amount > 0) && stock.amount < 0 {
            Err(DBError::OO())
        } else if stock.amount < 0 {
            //add cash amount
            let res = self.share_buy(stock).await?;
            let amount = stock.amount as f64 * stock.price;

            let cash = Cash {
                id: None,
                currency: String::from("eur"),
                amount,
                //TODO no unwrap
                owner: Thing::from_str("user:testuser1").unwrap(),
                timestamp: stock.datebuy.to_string(),
            };

            let cashadded = self.cash_entry(&cash).await?;
            Ok(res)
        } else {
            Err(DBError::Sdb)
        }
    }
    #[allow(unused)]
    pub async fn shares_mean_price(&self, symbol: &str) -> Result<f64, DBError> {
        let query = format!(
            "SELECT price, amount FROM share WHERE symbol = '{}';",
            symbol,
        );
        let mut result = self.db.query(query).await?;
        let shares: Vec<shares_price> = result.take(0)?;
        let total: f64 = shares.into_iter().map(|s| s.price * s.amount as f64).sum();
        Ok(total)
    }

    #[allow(unused)]
    pub async fn shares_select(&self, tb: &str, s: &Stock) -> Result<Vec<Stock>, DBError> {
        let query = format!(
            "SELECT id FROM {} WHERE symbol = '{}' AND 
            amount = {} AND price = {} AND owner = '{}' AND
             datebuy = '{}';",
            tb, s.symbol, s.amount, s.price, s.owner, s.datebuy
        );
        print!("{}", query);
        let mut result = self.db.query(query).await?;
        let shares: Vec<Stock> = result.take(0)?;
        Ok(shares)
    }

    #[allow(unused)]
    pub async fn delte_entry(&self, keyword: &str) -> Result<User, DBError> {
        let query = format!("DELETE '{}';", keyword);
        let mut result = self.db.query(query).await?;
        let shares: Vec<Stock> = result.take(0)?;
        todo!()
    }

    /*  #[allow(unused)]
    async fn stock_sells(&self, stock: &Stock) -> surrealdb::Result<()> {
        /* let mut result = self
        .db
        .query("SELECT * FROM cashsum WHERE owner = 'user:testuser1';")
        .await?; */

        let mut minus = "-".to_owned();
        let amount = &stock.amount;
        minus.push_str(amount);
        let set1: Vec<(&str, &str)> = vec![
            ("name", &stock.name),
            ("symbol", &stock.symbol),
            ("amount", &minus),
            ("owner", &stock.owner),
        ];

        //SELECT user FROM events WHERE type = 'activity' GROUP ALL;
        let i = vec!["symbol"];
        let ii = vec!["shares"];

        //let cond = stock.symbol.to_owned();
        let cond = format!("{} = {}", "symbol", stock.symbol.to_owned());

        let iii = vec![cond.as_str()];

        let set2: Vec<(&str, Vec<&str>)> = vec![("SELECT", i), ("FROM", ii), ("WHERE", iii)];
        let oo = create_select(&set2);

        let mut result = self.db.query(oo).await?;
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
        create_update(&rpg_party);

        Ok(())
    } */

    /*  #[allow(unused)]
    async fn stock_buys<'q>(&self, stock: &Stock) -> surrealdb::Result<()> {
        //println!("{}", "++++++++++++++++++++++");
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
    } */
}
#[cfg(test)]
mod tests {

    use std::time::SystemTime;

    use chrono::{DateTime, Utc};
    //TODO TIMESTAMP REALTIME
    use surrealdb::sql::{Id, Thing};

    use crate::{cash::Cash, initdb};

    use super::*;

    #[tokio::test]
    async fn stock_lifecycle() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };

        let now: DateTime<Utc> = Utc::now();
        println!("{}", now.to_rfc3339());

        let share = Stock {
            id: None,
            stock: Thing {
                tb: String::from("stock"),
                id: Id::from("bat"),
            },
            name: String::from("British American Tobacco"),
            symbol: String::from("bat"),
            amount: 5,
            price: 22.22, //String::from("22.22"),
            owner: Thing {
                tb: String::from("user"),
                id: Id::from("testuser1"),
            },
            datebuy: String::from("2023-07-03T07:18:52Z"),
        };
        let now = Utc::now();
        let formatted = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let cash = Cash {
            id: None,
            currency: String::from("eur"),
            amount: 200.0,
            owner: Thing {
                tb: String::from("user"),
                id: Id::from("testuser1"),
            },
            timestamp: formatted,
        };

        let cashadded = ii.cash_entry(&cash).await?;
        assert!(cashadded.id.tb == "cash");

        let test = ii.share_buy(&share).await?;
        assert!(test);

        let shares = ii.shares_select("share", &share).await?;
        assert!(shares.len() > 0);

        let shares = ii.share_sell(&share).await?;
        assert!(shares);
        Ok(())
    }

    #[tokio::test]
    async fn stock_buy() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };

        let share = Stock {
            id: id,
            stock: Thing {
                tb: String::from("stock"),
                id: Id::from("bat"),
            },
            name: String::from("British American Tobacco"),
            symbol: String::from("bat"),
            amount: 110000,
            price: 22.22, //String::from("22.22"),
            owner: Thing {
                tb: String::from("user"),
                id: Id::from("testuser1"),
            },
            datebuy: String::from("2023-07-03T07:18:52Z"),
        };

        let test = ii.share_buy(&share).await?;

        Ok(())
    }
    #[tokio::test]
    async fn stock_sum() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };
        let shares = ii.share_sum("bat").await?;

        println!("{}", shares);
        Ok(())
    }

    #[tokio::test]
    async fn stock_sell() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };
        let share = Stock {
            id: None,
            stock: Thing {
                tb: String::from("stock"),
                id: Id::from("bat"),
            },
            name: String::from("British American Tobacco"),
            symbol: String::from("bat"),
            amount: -110000,
            price: 22.22, //String::from("22.22"),
            owner: Thing {
                tb: String::from("user"),
                id: Id::from("testuser1"),
            },
            datebuy: String::from("2023-07-03T07:18:52Z"),
        };

        let shares = ii.share_sell(&share).await?;

        //println!("{}", shares);
        Ok(())
    }

    #[tokio::test]
    async fn stock_price_mean() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };

        let shares = ii.shares_mean_price("bat").await?;

        //println!("{}", shares);
        Ok(())
    }
    #[tokio::test]
    async fn stock_select() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };
        let share = Stock {
            id: None,
            stock: Thing {
                tb: String::from("stock"),
                id: Id::from("bat"),
            },
            name: String::from("British American Tobacco"),
            symbol: String::from("bat"),
            amount: 110000,
            price: 22.22, //String::from("22.22"),
            owner: Thing {
                tb: String::from("user"),
                id: Id::from("testuser1"),
            },
            datebuy: String::from("2023-07-03T07:18:52Z"),
        };

        let shares = ii.shares_select("share", &share).await?;

        println!("{:?}", shares);
        Ok(())
    }

    #[tokio::test]
    async fn entry_delete() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };

        let shares = ii.delte_entry("share:test3").await?;

        //println!("{}", shares);
        Ok(())
    }

    #[tokio::test]
    async fn flush_db() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };

        let shares = ii.flushdb("cash").await?;

        //println!("{}", shares);
        Ok(())
    }
}
