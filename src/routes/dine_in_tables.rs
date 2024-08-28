
use actix_web::{Responder, web::{Data, Json}, get, post, HttpResponse};
use sqlx::{MySqlPool, Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DineInRequest {
    table_number: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TableUsageError {
    error: String,
}


#[post("/use_table")]
pub async fn use_table(db: Data<MySqlPool>, body: Json<DineInRequest>) -> impl Responder {
    // if not exists insert
    let response = sqlx::query("UPDATE `dine_in_tables` set table_status = 'OCCUPIED' where
    `table_number` = ?")
    .bind(&body.table_number)
    .execute(&**db)
    .await;

    match response {
        Ok(res) => { HttpResponse::Created().json(DineInRequest{
            table_number: body.table_number,
            })},
        Err(_e) => HttpResponse::InternalServerError().json(TableUsageError{
            error: _e.to_string(),
            })
        }
}

// TODO -> it should calculate total bill
#[post("/check_out_table")]
pub async fn check_out_table(db: Data<MySqlPool>, body: Json<DineInRequest>) -> impl Responder {
    // DELETE FROM `order` WHERE table_number = ?

     let response = sqlx::query("UPDATE `dine_in_tables` set table_status = 'VACANT' where
        `table_number` = ?")
        .bind(&body.table_number)
        .execute(&**db)
        .await;

        match response {
            Ok(res) => { HttpResponse::Created().json(DineInRequest{
                table_number: body.table_number,
                })},
            Err(_e) => HttpResponse::InternalServerError().json(TableUsageError{
                error: _e.to_string(),
                })
            }
}

