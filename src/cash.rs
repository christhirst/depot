use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{DBError, Record, User, DB};

//TODO CASH ADD
//TODO BUY C
//TODO SELL C

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct cash {
    pub currency: String,
    pub amount: f64,
    pub owner: Thing,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cashsum {
    pub currency: String,
    pub owner: Thing,
    pub sum: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct shares_price {
    #[allow(dead_code)]
    amount: i64,
    price: f64,
}

impl<'s> DB<'s> {
    #[allow(unused)]
    pub async fn cash_entry(&self, c: &cash) -> Result<Record, DBError> {
        //SELECT * FROM cash WHERE owner='users:Tobie@web.de' AND currency='eur';
        let query =
            format!(
            "CREATE cash SET currency = '{}',  amount = {}, owner = {}, timestamp = <datetime>'{}';",
            c.currency, c.amount.to_string(), c.owner, c.timestamp,
        );
        let mut resp = self.db.query(query).await?;
        let rec: Option<Record> = resp.take(0)?;
        let rec = rec.ok_or(DBError::Sdb)?;
        Ok(rec)
    }
    #[allow(unused)]
    pub async fn cash_remove(&self, table: &str) -> Result<bool, DBError> {
        //SELECT * FROM cash WHERE owner='users:Tobie@web.de' AND currency='eur';
        //DELETE cash:id;

        Ok(true)
    }
    #[allow(unused)]
    pub async fn cash_add(&self, user: &User) -> Result<User, DBError> {
        todo!()
    }

    #[allow(unused)]
    pub async fn cash_get(&self, table: &str) -> surrealdb::Result<()> {
        //SELECT * FROM cash WHERE owner='users:Tobie@web.de' AND currency='eur';

        Ok(())
    }
    #[allow(unused)]
    pub async fn sum_get(&self, table: &str) -> surrealdb::Result<()> {
        //SELECT * FROM cash WHERE owner='users:Tobie@web.de' AND currency='eur';
        let mut result: surrealdb::Response = self
        .db
        .query("return (SELECT total from (SELECT math::sum(amount) AS total, currency FROM cash GROUP BY currency));")
        .await?;
        Ok(())
    }

    pub async fn cash_sum(&self, owner: &str, currency: &str) -> Result<Cashsum, DBError> {
        //SELECT math::sum(amount) AS sum,symbol FROM share WHERE symbol = 'bat' GROUP BY symbol;

        let query = format!(
            "SELECT math::sum(amount) AS sum, currency, owner FROM cash WHERE currency = '{}' AND owner = '{}' GROUP BY currency, owner;",
            currency,
            owner
        );

        let mut result = self.db.query(query).await?;

        let shares: Option<Cashsum> = result.take(0)?;
        let w = shares.ok_or(DBError::Sdb)?;
        Ok(w)
    }
}

#[cfg(test)]
mod tests {
    use surrealdb::sql::{Id, Thing};

    use crate::initdb;

    use super::*;
    #[tokio::test]
    async fn cash_lifecycle() -> Result<(), DBError> {
        //TODO CASH ADD
        //TODO CASH ADD

        let db = initdb("e").await?;
        let db = DB { db: &db };
        let (owner, currency) = ("user:testuser1", "eur");
        let cashsum = db.cash_sum(owner, currency).await?;

        println!("{:?}", cashsum);
        //TODO CASH DELETE
        Ok(())
    }
}
