use crate::ctx::Ctx;
use crate::error::Resultc;
//use crate::middleware::middleware_header;
use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use db_service::model::{Stock, StockEntry};

use db_service::User;
use serde::{Deserialize, Serialize};
//use db_service::model::Stock;
use surrealdb::sql::{Id, Thing};
use tracing::debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TbId {
    tb: String,
    id: String,
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        //.route_layer(middleware::from_fn(f))
        .route("/user", post(stock_add))
        .route("/stock", post(stock_add))
        .route("/stock/:id", get(stock_list))
        .route("/stock/:id", delete(entry_del))
        .route("/shares_buy", post(stock_buy))
        .route("/shares_sell/", post(stock_sell))
        .route("/shares_get/:id", get(trade_get))
        .route("/shares_list", get(stock_list))
        .route("/shares_amount_by_symbol/:id", get(stock_list))
        .route("/shares_stock_worth_by_symbol/:id", get(stock_list))
        //entry
        .route("/entry", delete(entry_del))
        //cash
        .route("/cash_add/", post(stock_list))
        .route("/cash_withdraw/", post(stock_list))
        .route("/cash_amount_by_currency/:id", post(stock_list))
        .route(
            "/cash_sum/",
            get(stock_list).post(stock_list).delete(stock_list),
        )
        .with_state(mc)
}

// region:    --- STOCK REST Handlers
async fn stock_add(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(stock): Json<StockEntry>,
) -> Resultc<Json<StockEntry>> {
    debug!("->> {:?} - Stock add", stock);
    Ok(Json(mc.stock_add(ctx, stock).await?))
}

async fn stock_list(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<String>,
) -> Resultc<Json<Vec<StockEntry>>> {
    debug!("->> {:?} - Stock add", id);

    Ok(Json(mc.stock_list(ctx, id).await?))
}

async fn stock_buy(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(stock): Json<Stock>,
) -> Resultc<Json<Stock>> {
    println!("->> {:?} - Stock buy", stock);
    Ok(Json(mc.stock_buy(ctx, stock).await?))
}

async fn stock_sell(
    State(mc): State<ModelController>,
    ctx: Ctx,
    //Path(id): Path<u64>,
    Json(stock): Json<Stock>,
) -> Resultc<Json<Stock>> {
    println!("->> {:?} - Stock sell", stock);

    Ok(Json(mc.stock_sell(ctx, stock).await?))
}
// endregion: --- STOCK REST Handlers

// region:    --- SHARES REST Handlers
async fn trade_get(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(_id): Path<u64>,
    Json(stock): Json<Stock>,
) -> Resultc<Json<Stock>> {
    println!(">>> {:<12} - trade_get", "HANDLER");

    let mut ticket = mc.stock_get(ctx, stock).await?;

    Ok(Json(ticket.remove(0)))
}

async fn share_list(
    State(_mc): State<ModelController>,
    _ctx: Ctx,
    Path(_id): Path<u64>,
    Json(_stock): Json<Stock>,
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
// endregion: --- SHARES REST Handlers

// region:    --- CASH REST Handlers
async fn cash_add(
    State(_mc): State<ModelController>,
    _ctx: Ctx,
    Json(_cash): Json<TicketForCreate>,
) -> Resultc<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    //let c = mc.cash_add(ctx, stock).await?;

    //let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    //Ok(Json(ticket))
    todo!()
}

async fn cash_sum_by_currency(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(currency): Json<String>,
) -> Resultc<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    //let c = mc.cash_add(ctx, stock).await?;

    let ticket = mc.cash_sum_by_currency(ctx, &currency).await?;

    //Ok(Json(ticket))
    todo!()
}
// endregion:    --- CASH REST Handlers

// region:    --- ENTRY REST Handlers
async fn entry_del(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(tbid): Json<TbId>,
) -> Resultc<Json<StockEntry>> {
    let _ = tbid;
    debug!("->> {:?} - Stock deleted", tbid);
    let mut ticket = mc.entry_del(ctx, (tbid.tb, tbid.id)).await?;

    Ok(Json(ticket))
}
// endregion: --- ENTRY REST Handlers

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
