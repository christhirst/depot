use std::any::Any;

use crate::ctx::Ctx;
use crate::error::{Error, Resultc};
//use crate::middleware::middleware_header;
use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{middleware, Json, Router};
use chrono::DateTime;
use db_service::model::Stock;
//use db_service::model::Stock;
use surrealdb::sql::Thing;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        //.route_layer(middleware::from_fn(f))
        .route("/stock_buy", post(stock_buy))
        .route("/stock_sell/:id", delete(stock_sell))
        .route("/stock_get/:id", get(stock_get))
        .route("/stock_list/", get(stock_list))
        .route(
            "/cash/",
            get(stock_list).post(stock_list).delete(stock_list),
        )
        .with_state(mc)
}

// region:    --- REST Handlers

async fn stock_buy(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(stock): Json<Stock>,
) -> Resultc<Json<Stock>> {
    println!("->> {:?} - Stock buy", stock);
    //let id = ctx.user_id();
    let o = "tb";
    let owner = Thing::from((o, "user"));
    let result = mc.stock_buy(ctx, stock).await?;

    Ok(Json(result))
}

async fn stock_sell(
    State(mc): State<ModelController>,
    ctx: Ctx,
    //Path(id): Path<u64>,
    Json(stock): Json<Stock>,
) -> Resultc<Json<Stock>> {
    println!("->> {:?} - Stock sell", stock);
    let res = mc.stock_sell(ctx, stock).await?;

    Ok(Json(res))
    //todo!()
}

async fn stock_list(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(stock): Json<Stock>,
) -> Resultc<Json<Vec<Stock>>> {
    /*  let id = surrealdb::sql::Id::String(String::from("ID"));
    let t = Thing {
        tb: "tb".to_string(),
        id: id,
    };
    println!("->> {:?} - ", t);

    println!("->> {:<12} - list_tickets", "HANDLER");
    let o = "tb";
    let tickets = mc.stock_sell(ctx, stock).await?; */

    //Ok(Json(tickets))
    todo!()
}

async fn stock_get(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Resultc<Json<Ticket>> {
    println!(">>> {:<12} - delete_ticket", "HANDLER");

    //let ticket = mc.delete_ticket(ctx, id).await?;

    //Ok(Json(ticket))
    todo!()
}

async fn cash_add(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(cash): Json<TicketForCreate>,
) -> Resultc<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    //let c = mc.cash_add(ctx, stock).await?;

    //let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    //Ok(Json(ticket))
    todo!()
}
// endregion: --- REST Handlers

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[tokio::test]
    async fn user_delete() {
        let ne: DateTime<Utc> = String::from("2023-07-03T07:18:52Z").parse().unwrap();
        println!("{}", ne.format("%Y-%m-%dT%H:%M:%SZ").to_string());
        //Mon, 3 Jul 2023 07:18:52 +0000
        //2023-07-03 07:18:52 UTC
        //2023-07-03T07:18:52Z
        //2023-07-03T07:18:52Z
    }
}
