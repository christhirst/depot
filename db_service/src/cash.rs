use crate::model::Cash;
use crate::{db_helper::thing_to_string, model::DBError, Record, User, DB};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

//TODO CASH ADD
//TODO BUY C
//TODO SELL C

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cashsum {
    pub currency: String,
    pub owner: Thing,
    pub sum: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SharesPrice {
    #[allow(dead_code)]
    amount: i64,
    price: f64,
}

impl DB {
    #[allow(unused)]
    pub async fn cash_entry(&self, c: &Cash) -> Result<Record, DBError> {
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
    pub async fn cash_remove(&self, t: &Thing) -> Result<Cash, DBError> {
        let mut resp: Option<Cash> = self.db.delete((t.id.to_string(), t.tb.to_owned())).await?;
        resp.ok_or(DBError::Sdb)
    }

    #[allow(unused)]
    pub async fn cash_get_by_currency(&self, c: &Cash) -> Result<Vec<Cash>, DBError> {
        //SELECT * FROM cash WHERE owner='users:Tobie@web.de' AND currency='eur';
        //SELECT * FROM cash WHERE 'eur' INSIDE  currency;
        let query = format!(
            "SELECT * FROM cash WHERE owner = '{}' AND {} INSIDE currency ;",
            c.owner, c.currency
        );
        let mut resp = self.db.query(query).await?;
        let res = resp.take(0)?;
        Ok(res)
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

    pub async fn cash_sum(&self, owner: &Thing, currency: &str) -> Result<Cashsum, DBError> {
        //SELECT math::sum(amount) AS sum,symbol FROM share WHERE symbol = 'bat' GROUP BY symbol;
        let owner = thing_to_string(owner.clone());
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
        let currency = "eur";
        let owner = Thing {
            tb: String::from("user"),
            id: Id::from("testuser1"),
        };
        let cashsum = db.cash_sum(&owner, currency).await?;

        println!("{:?}", cashsum);
        //TODO CASH DELETE
        Ok(())
    }
}
