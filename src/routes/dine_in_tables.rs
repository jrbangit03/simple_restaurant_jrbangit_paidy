
use actix_web::{Responder, web::{Data, Json}, get, post, HttpResponse};
use sqlx::{MySqlPool};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DineInRequest {
    table_number: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AddTable {
    table_number: i32,
    table_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct TableUsageError {
    error: String,
}

#[post("/add_table")]
pub async fn add_table(db: Data<MySqlPool>, body: Json<AddTable>) -> impl Responder {
    let response = sqlx::query("INSERT INTO `dine_in_tables` (`table_number`, `table_status`) VALUES (?, ?)")
    .bind(&body.table_number)
    .bind(&body.table_status)
    .execute(&**db)
    .await;

    match response {
        Ok(_res) => { HttpResponse::Created().json(AddTable{
            table_number: body.table_number.clone(),
            table_status: body.table_status.clone(),
            })},
        Err(_e) => HttpResponse::InternalServerError().json(TableUsageError{
            error: _e.to_string(),
            })
        }
}


#[post("/use_table")]
pub async fn use_table(db: Data<MySqlPool>, body: Json<DineInRequest>) -> impl Responder {
    let response = sqlx::query("UPDATE `dine_in_tables` set table_status = 'OCCUPIED' where
    `table_number` = ?")
    .bind(&body.table_number)
    .execute(&**db)
    .await;

    match response {
        Ok(_res) => { HttpResponse::Created().json(DineInRequest{
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
            Ok(_res) => { HttpResponse::Created().json(DineInRequest{
                table_number: body.table_number,
                })},
            Err(_e) => HttpResponse::InternalServerError().json(TableUsageError{
                error: _e.to_string(),
                })
            }
}

