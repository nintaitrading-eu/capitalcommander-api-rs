use crate::schema::t_account;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct AccountList(pub Vec<Account>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Account
{
    pub account_id: i64,
    pub name: String,
    pub description: String,
    pub is_deleted: bool,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name="t_account"]
pub struct AccountNew
{
    pub name: String,
    pub description: String,
}
