use actix_web::web;
use crate::schema::t_account::dsl::*;
use crate::models::accountmodel::{Account, AccountNew};
use crate::dto::accountdto::AccountDto;
use crate::connection::Pool;

use crate::diesel::dsl::{delete, insert_into, update};
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use chrono::Utc;

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

pub fn add_account(
    db: web::Data<Pool>,
    item: web::Json<AccountDto>) -> Result<Account, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let new_account = AccountNew {
        name: &item.name,
        description: &item.description,
        is_deleted: false,
        date_created: Utc::now().naive_utc(),
        date_modified: Utc::now().naive_utc(),
    };
    let res = insert_into(t_account).values(&new_account).get_result(&conn)?;
    Ok(res)
}

pub fn delete_account(db: web::Data<Pool>, id: i64) -> Result<usize, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let count = delete(t_account.find(id)).execute(&conn)?;
    Ok(count)
}

pub fn update_account(
    db: web::Data<Pool>,
    id: i64,
    item: web::Json<AccountDto>) -> Result<Account, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let updated_account = update(t_account.filter(account_id.eq(id)))
        .set((name.eq(&item.name),
            description.eq(&item.description),
            is_deleted.eq(false),
            date_modified.eq(Utc::now().naive_utc())))
        .get_result(&conn)?;
    Ok(updated_account)
}
