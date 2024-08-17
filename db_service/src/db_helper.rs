use surrealdb::{
    engine::{
        any::Any,
        remote::ws::{Client, Ws},
    },
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

use crate::{model::DBError, DB};

pub fn thing_to_string(id: Thing) -> String {
    format!("{}:{}", id.tb, id.id)
}

fn string_wrap(s: &str) -> String {
    format!("'{}'", s)
}

pub fn define_table(table: &Vec<&str>) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("DEFINE TABLE {} SCHEMAFULL; ", s);
        q.push_str(&qs)
    }
    q
}

pub fn define_field(table: &[(&str, &str, &str)]) -> String {
    let mut q = String::from("");
    for s in table {
        let qs = format!("DEFINE FIELD {} ON TABLE {} TYPE {};", s.0, s.1, s.2);
        q.push_str(&qs)
    }
    q
}

pub async fn initdb(s: &str) -> Result<Surreal<Any>, DBError> {
    let _ = s;
    let db: Surreal<Client>;

    /* if s == "mem" {
        //db = surrealdb::engine::any::connect("mem://").await?;
    } else {
        db = Surreal::new::<Ws>("0.0.0.0:8080").await?;
    } */
    let db = surrealdb::engine::any::connect("ws://0.0.0.0:8080").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;
    Ok(db)
}

// impl of Val
impl DB {
    pub async fn db_init(
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
