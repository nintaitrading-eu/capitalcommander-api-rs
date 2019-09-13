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

impl AccountList
{
    pub fn list() -> Self
    {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::t_account::dsl::*;
        use crate::connection::establish_connection;

        let connection = establish_connection();

        let result =
            t_account
                .limit(1)
                .load::<Account>(&connection)
                .expect("Error loading account");

        AccountList(result)
    }
}
