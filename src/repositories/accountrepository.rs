use actix_web::web;
use crate::schema::t_account::dsl::*;
use crate::models::accountmodel::Account;
use crate::connection::Pool;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

pub fn get_account_list(db: web::Data<Pool>) -> Result<Vec<Account>, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let records = t_account.load::<Account>(&conn)?; 
    Ok(records)
}

pub fn get_account_by_id(db: web::Data<Pool>, id: i64) -> Result<Account, diesel::result::Error>
{
    let conn = db.get().unwrap();
    t_account.find(id).get_result::<Account>(&conn)
}
