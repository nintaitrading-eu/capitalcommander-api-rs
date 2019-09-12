/*use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow, Queryable};
use diesel::row::Row;
use diesel::sql_types::Text;
*/
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
