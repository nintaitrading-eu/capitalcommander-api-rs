use crate::schema::t_trade;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

#[derive(Serialize, Deserialize)]
pub struct TradeList(pub Vec<Trade>);

#[derive(Queryable, Serialize, Deserialize, Identifiable, AsChangeset)]
#[primary_key(trade_id)]
#[table_name="t_trade"]
pub struct Trade
{
    pub trade_id: i64,
    pub trade_calculated_id: i64,
    pub product_id: i64,
    pub pool_id: i64,
    pub trade_cost_id: i64,
    pub trade_drawdown_id: i64,
    pub date_buy: Option<NaiveDateTime>,
    pub year_buy: Option<i32>,
    pub month_buy: Option<i32>,
    pub day_buy: Option<i32>,
    pub date_sell: Option<NaiveDateTime>,
    pub year_sell: Option<i32>,
    pub month_sell: Option<i32>,
    pub day_sell: Option<i32>,
    pub shares_buy: i32,
    pub shares_sell: i32,
    pub price_buy: BigDecimal,
    pub price_sell: BigDecimal,
    pub commission_buy: BigDecimal,
    pub commission_sell: BigDecimal,
    pub tax_buy: BigDecimal,
    pub tax_sell: BigDecimal,
    pub is_deleted: bool,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name="t_trade"]
pub struct TradeNew<'a>
{
    pub trade_calculated_id: i64,
    pub product_id: i64,
    pub pool_id: i64,
    pub trade_cost_id: i64,
    pub trade_drawdown_id: i64,
    pub date_buy: Option<NaiveDateTime>,
    pub year_buy: Option<i32>,
    pub month_buy: Option<i32>,
    pub day_buy: Option<i32>,
    pub date_sell: Option<NaiveDateTime>,
    pub year_sell: Option<i32>,
    pub month_sell: Option<i32>,
    pub day_sell: Option<i32>,
    pub shares_buy: i32,
    pub shares_sell: i32,
    pub price_buy: &'a BigDecimal,
    pub price_sell: &'a BigDecimal,
    pub commission_buy: &'a BigDecimal,
    pub commission_sell: &'a BigDecimal,
    pub tax_buy: &'a BigDecimal,
    pub tax_sell: &'a BigDecimal,
    pub is_deleted: bool,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}
