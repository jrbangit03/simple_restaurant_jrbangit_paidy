
use actix_web::{Responder, post, web::{Json, Data}, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, MySqlPool};


#[derive(Serialize, Deserialize)]
pub struct CreateNewOrder {
    table_number: i32,
    food_id: i32,
    food_name: String,
    quantity: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Order {
    order_id: i32,
    table_number: i32,
    food_id: i32,
    food_name: String,
    quantity: i32,
}

#[derive(Serialize)]
pub struct OrderError {
    error: String,
}


#[post("/order/create")]
pub async fn create_new_order(db: Data<MySqlPool>, body: Json<CreateNewOrder>) -> impl Responder{
    let response = sqlx::query(
        "INSERT INTO `order` (`table_number`, `food_id`, `food_name`, `quantity`) values (?, ?, ?, ?) ")
        .bind(&body.table_number)
        .bind(&body.food_id)
        .bind(&body.food_name)
        .bind(&body.quantity)
        .execute(&**db)
        .await;

    match response {
        Ok(id) => {
            HttpResponse::Created().json(Order {
                order_id: id.last_insert_id() as i32,
                table_number: body.table_number.clone(),
                food_id: body.food_id.clone(),
                food_name: body.food_name.clone(),
                quantity: body.quantity.clone(),
                })
        },
        Err(_e) => HttpResponse::InternalServerError().json(OrderError{
            error: _e.to_string(),
        }),
    }


}

// #[get("/order/{table_number}")]
// pub async fn get_orders_by_table() -> {
//
//
// }