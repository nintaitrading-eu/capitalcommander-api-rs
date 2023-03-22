use crate::schema::t_account;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct AccountList(pub Vec<Account>);

#[derive(Queryable, Serialize, Deserialize, Identifiable, AsChangeset)]
#[primary_key(account_id)]
#[table_name="t_account"]
pub struct Account
{
    pub account_id: i64,
    pub name: String,
    pub description: String,
    pub is_deleted: bool,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name="t_account"]
pub struct AccountNew<'a>
{
    pub name: &'a str,
    pub description: &'a str,
    pub is_deleted: bool,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}
