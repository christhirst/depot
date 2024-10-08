use crate::model::DBError;
use crate::DB;
use crate::{create_entries, db_helper::thing_to_string, User};
use serde::Serialize;
use std::collections::HashMap;
use surrealdb::opt::auth::Database;
use surrealdb::opt::auth::Record;
#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
pub struct DbConf {
    pub namespace: String,
    pub database: String,
    pub scope: String,
}

impl DB {
    #[allow(unused)]
    pub async fn user_signup(&self, db_conf: DbConf, user: &User) -> Result<String, DBError> {
        let jwt = self
            .db
            .signup(Record {
                namespace: &db_conf.namespace,
                database: &db_conf.database,
                //scope: &db_conf.scope,
                params: Credentials {
                    email: &user.mail,
                    password: &user.pw,
                },
                //TODO Softcoding
                access: "access",
            })
            .await?;

        Ok(jwt.as_insecure_token().to_string())
    }

    #[allow(unused)]
    pub async fn user_signin(&self, db_conf: DbConf, user: &User) -> Result<String, DBError> {
        let jwt = self
            .db
            .signin(Database {
                namespace: &db_conf.namespace,
                database: &db_conf.database,
                /* scope: &db_conf.scope,
                params: Credentials {
                    email: &user.mail,
                    password: &user.pw,
                }, */
                username: &user.mail,
                password: &user.pw,
            })
            .await?;
        Ok(jwt.as_insecure_token().to_string())
    }

    /* #[allow(unused)]
    pub async fn user_add(&self, user: &User) -> Result<User, DBError> {
        let set1: Vec<(&str, &str)> =
            vec![("name", &user.name.clone().unwrap()), ("mail", &user.mail)];
        let mut rpg_party = HashMap::new();
        let id: &str = &thing_to_string(user.clone().id.unwrap());
        rpg_party.insert(id, set1);

        let query = create_entries(&rpg_party);

        let mut result = self.db.query(query).await?;
        let pp: Option<User> = result.take(0)?;
        pp.ok_or(DBError::Sdb)
    } */

    pub async fn user_del_by_id(&self, name: &str) -> surrealdb::Result<()> {
        let query = format!("{} {};", "DELETE", name);
        self.db.query(query).await?;

        Ok(())
    }

    async fn user_search_by_id(&self, name: &str) -> Result<User, DBError> {
        //SELECT * FROM user:testuser1;
        let query = format!("SELECT * FROM {}", name);
        let mut result = self.db.query(query).await?;
        let user: Option<User> = result.take(0)?;
        match user {
            Some(u) => Ok(u),
            None => Err(DBError::Sdb),
        }
    }
    async fn user_update_by_id(&self, u: &User) -> Result<User, DBError> {
        //UPDATE user:testuser1 SET name = 'Tobie', mail = 'SurrealDB@mail';
        let query = format!(
            "UPDATE {} SET name = '{}', mail = '{}';",
            thing_to_string(u.id.clone().unwrap()),
            u.clone().name.unwrap(),
            u.mail
        );
        println!("{query}");
        let mut result = self.db.query(query).await?;
        let user: Option<User> = result.take(0)?;

        Ok(user.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use surrealdb::sql::{Id, Thing};

    use crate::initdb;

    use super::*;

    /*  #[tokio::test]
    async fn create_entry() -> Result<(), DBError> {
        //init db
        let db = initdb("mem").await?;
        let ii = DB { db: &db };
        //create id
        let t = Thing {
            id: Id::from("testuser3"),
            tb: From::from("user"),
        };
        //create user
        let user = User {
            id: t.clone(),
            name: String::from("'testuser2'"),
            mail: String::from("'testuser2@mail'"),
            pw: String::from("'testuser2'"),
        };
        let uu = ii.user_add(&user).await?;

        let id = thing_to_string(uu.id.clone());
        let user_got = ii.user_search_by_id(&id).await?;
        println!("{user_got:?}");

        //delete user
        ii.user_del_by_id(&id).await?;

        //user update
        let user = User {
            id: t.clone(),
            name: String::from("testuser2"),
            mail: String::from("testuser1@mail"),
            pw: String::from("'testuser2'"),
        };
        //update user
        ii.user_update_by_id(&user).await?;
        ii.user_del_by_id(&id).await?;

        Ok(())
    } */

    /* #[ignore]
    #[tokio::test]
    async fn user_select() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };
        let tb_user1 = String::from("user:testuser1");
        let user_got = ii.user_search_by_id(&tb_user1).await?;
        let i = Id::from("testuser1");
        let t = Thing {
            id: i,
            tb: "user".to_owned(),
        };
        let user = User {
            id: t,
            name: String::from("testuser1"),
            mail: String::from("testuser1@mail"),
            pw: String::from("'testuser2'"),
        };
        println!("{user_got:?}");
        println!("{user:?}");
        assert!(user == user_got);
        Ok(())
    } */

    /* #[ignore]
    #[tokio::test]
    async fn user_update() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };
        let tb_user1 = String::from("user:testuser1");
        let user_got = ii.user_search_by_id(&tb_user1).await?;
        let t = Thing {
            id: Id::from("testuser1"),
            tb: From::from("user"),
        };
        let user = User {
            id: t,
            name: String::from("testuser2"),
            mail: String::from("testuser1@mail"),
            pw: String::from("'testuser2'"),
        };

        ii.user_update_by_id(&user).await?;
        assert!(user == user_got);
        Ok(())
    } */

    #[ignore]
    #[tokio::test]
    async fn user_delete() -> Result<(), DBError> {
        let db = initdb("e").await?;
        let ii = DB { db: &db };
        let tb_user1 = String::from("user:testuser1");
        ii.user_del_by_id(&tb_user1).await?;

        Ok(())
    }
}
