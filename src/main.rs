use error::Error;
/* use db_helper::initdb;
use model::DBError; */
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Scope;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
/* mod cash;
mod db_helper;
mod stock;
mod user; */
mod ctx;
mod error;

mod model;
mod web;

use crate::model::ModelController;
use axum::{
    extract::Query,
    response::{Html, IntoResponse, Response},
    routing::get,
    Extension, Router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let db = db_service::db_helper::initdb("e").await?;
    //let db = surrealdb::engine::any::connect("mem://").await.unwrap();
    db.use_ns("test").use_db("test").await?;
    let db = db_service::DB { db };

    //init tables
    let table = vec!["user", "cash", "share", "cashsum", "stock"];

    //init fields
    let set: Vec<(&str, &str, &str)> = vec![
        //user
        /* ("password", "user", "string"),
        ("mail", "user", "string"), */
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
        ("stock", "share", "record(stock)"),
        ("name", "share", "string"),
        ("owner", "share", "record(user)"),
        ("symbol", "share", "string"),
        ("amount", "share", "number"),
        ("price", "share", "number"),
        ("datebuy", "share", "datetime"),
        //stock
        ("name", "stock", "string"),
        ("wkn", "stock", "string"),
        ("isin", "stock", "string"),
        ("symbol", "stock", "string"),
        ("country", "stock", "string"),
    ];

    let idx = vec![("symbolIndex", "stock", "symbol")];

    let _u = db.db_init(&table, &set, &idx).await?;

    /* let jwt = db
        .db
        .signup(Scope {
            namespace: "test",
            database: "test",
            scope: "access",
            params: Credentials {
                email: "admin2 @test.de",
                password: "test",
            },
        })
        .await?;
    println!("jwt: {:?}", jwt); */
    //server start
    // Initialize ModelController.
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone());
    // .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes())
        .merge(web::routes_login::routes(mc.clone()))
        .nest("/api", routes_apis)
        .layer(Extension(mc))
        //.layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());
    //TODO fallback static

    info!("Starting Server");

    // region:    --- Start Server
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server
    println!("done");
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hallo(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("Hallo");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hallo {name}"))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "HANDLER");
    res
}

fn routes() -> Router {
    Router::new().route("/", get(handler_hallo))
}
