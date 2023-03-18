use actix_web::web;
use crate::schema::t_account::dsl::t_account;
use crate::models::accountmodel::Account;
use crate::connection::Pool;

use crate::diesel::RunQueryDsl;

pub fn get_account_list(db: web::Data<Pool>) -> Result<Vec<Account>, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let records = t_account.load::<Account>(&conn)?; 
    Ok(records)
}
