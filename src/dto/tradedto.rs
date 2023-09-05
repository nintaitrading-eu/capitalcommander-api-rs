use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeDto
{
    pub product_id: i64,
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
}
