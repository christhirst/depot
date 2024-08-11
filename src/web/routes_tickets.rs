use crate::ctx::Ctx;
use crate::error::{Error, Resultc};
//use crate::middleware::middleware_header;
use crate::model::{ModelController, Stock, Ticket, TicketForCreate};
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{middleware, Json, Router};
use chrono::DateTime;
//use db_service::model::Stock;
use surrealdb::sql::Thing;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        //.route_layer(middleware::from_fn(f))
        .route("/tickets", post(stock_buy).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}
// region:    --- REST Handlers
async fn cash_add(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Resultc<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    //let c = mc.cash_add(ctx, stock).await?;

    //let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    //Ok(Json(ticket))
    todo!()
}
async fn stock_buy(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(stock): Json<Stock>,
) -> Resultc<Json<Stock>> {
    println!("->> {:?} - list_tickets", stock);
    let o = "tb";
    let owner = Thing::from((o, "user"));
    let tickets = mc.get_cash_sum(&owner, "").await?;

    Ok(Json(stock))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Resultc<Json<Vec<Ticket>>> {
    let id = surrealdb::sql::Id::String(String::from("ID"));
    let t = Thing {
        tb: "tb".to_string(),
        id: id,
    };
    println!("->> {:?} - ", t);

    /*  println!("->> {:<12} - list_tickets", "HANDLER");
       let o = "tb";
       let owner = Thing::from((o, "user"));
       let tickets = mc.get_cash_sum(ctx, &owner, "").await?;
    */
    //Ok(Json(tickets))
    todo!()
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Resultc<Json<Ticket>> {
    println!(">>> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
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
