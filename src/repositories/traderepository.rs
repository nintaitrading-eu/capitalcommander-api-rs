use actix_web::web;
use crate::schema::t_trade::dsl::*;
use crate::models::trademodel::{Trade, TradeNew};
use crate::dto::tradedto::TradeDto;
use crate::connection::Pool;

use crate::diesel::dsl::{delete, insert_into, update};
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use chrono::Utc;

pub fn get_trade_list(db: web::Data<Pool>) -> Result<Vec<Trade>, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let records = t_trade.load::<Trade>(&conn)?; 
    Ok(records)
}

pub fn get_trade_by_id(db: web::Data<Pool>, id: i64) -> Result<Trade, diesel::result::Error>
{
    let conn = db.get().unwrap();
    t_trade.find(id).get_result::<Trade>(&conn)
}

pub fn add_trade(
    db: web::Data<Pool>,
    item: web::Json<TradeDto>) -> Result<Trade, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let new_trade = TradeNew {
        trade_calculated_id: 1 as i64, // TODO: Needs the trade_calculated_id, which first needs to be added.
        product_id: item.product_id,
        pool_id: 1 as i64, // TODO: Needs the trade_pool_id, perhaps hardcoded is enough?
        trade_cost_id: 1 as i64, // TODO: Needs the trade_cost_id
        trade_drawdown_id: 1 as i64, // TODO: Needs the trade_drawdown_id
        date_buy: item.date_buy,
        year_buy: item.year_buy,
        month_buy: item.month_buy,
        day_buy: item.day_buy,
        date_sell: item.date_sell,
        year_sell: item.year_sell,
        month_sell: item.month_sell,
        day_sell: item.day_sell,
        shares_buy: item.shares_buy,
        shares_sell: item.shares_sell,
        price_buy: &item.price_buy,
        price_sell: &item.price_sell,
        commission_buy: &item.commission_buy,
        commission_sell: &item.commission_sell,
        tax_buy: &item.tax_buy,
        tax_sell: &item.tax_sell,
        is_deleted: false,
        date_created: Utc::now().naive_utc(),
        date_modified: Utc::now().naive_utc(),
    };
    let res = insert_into(t_trade).values(new_trade).get_result(&conn)?;
    Ok(res)
}

pub fn delete_trade(db: web::Data<Pool>, id: i64) -> Result<usize, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let count = delete(t_trade.find(id)).execute(&conn)?;
    Ok(count)
}

pub fn update_trade(
    db: web::Data<Pool>,
    id: i64,
    item: web::Json<TradeDto>) -> Result<Trade, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let updated_trade = update(t_trade.filter(trade_id.eq(id)))
        .set((product_id.eq(item.product_id),
            date_buy.eq(item.date_buy),
            year_buy.eq(item.year_buy),
            month_buy.eq(item.month_buy),
            day_buy.eq(item.day_buy),
            date_sell.eq(item.date_sell),
            year_sell.eq(item.year_sell),
            month_sell.eq(item.month_sell),
            day_sell.eq(item.day_sell),
            shares_buy.eq(item.shares_buy),
            shares_sell.eq(item.shares_sell),
            price_buy.eq(&item.price_buy),
            price_sell.eq(&item.price_sell),
            commission_buy.eq(&item.commission_buy),
            commission_sell.eq(&item.commission_sell),
            tax_buy.eq(&item.tax_buy),
            tax_sell.eq(&item.tax_sell),
            is_deleted.eq(false),
            date_modified.eq(Utc::now().naive_utc())))
        .get_result(&conn)?;
    Ok(updated_trade)
}
