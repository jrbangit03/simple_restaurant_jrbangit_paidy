
use actix_web::{Responder, post, get, web::{Json, Data}, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, MySqlPool, Error};


// TODO -> CreateNewOrder should be impl as fn of Order struct
#[derive(Serialize, Deserialize)]
pub struct CreateNewOrder {
    table_number: i32,
    food_id: i32,
    food_name: String,
    quantity: i32,
}

// TODO -> DeleteOrder should be impl as fn of Order struct
#[derive(Serialize, Deserialize)]
pub struct DeleteOrder {
    table_number: i32,
    food_name: String,
}

// TODO -> ListOrder should be impl as fn of Order struct
#[derive(Serialize, Deserialize)]
pub struct ListOrder {
    table_number: i32,
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
        "INSERT INTO `order` (`table_number`, `food_id`, `food_name`, `quantity`)
         SELECT ?, ?, ?, ? FROM DUAL
         WHERE EXISTS (SELECT 1 FROM `dine_in_tables` WHERE `table_status`='OCCUPIED' AND `table_number`=?)
        ")
        .bind(&body.table_number)
        .bind(&body.food_id)
        .bind(&body.food_name)
        .bind(&body.quantity)
        .bind(&body.table_number)
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

#[post("/order/delete")]
pub async fn delete_order(db: Data<MySqlPool>, body: Json<DeleteOrder>) -> impl Responder {
    let response = sqlx::query(
            "DELETE FROM `order`
             WHERE EXISTS (SELECT 1 FROM `dine_in_tables` WHERE table_status='OCCUPIED' AND table_number=?)
             AND table_number=?
             AND food_name=?
            ")
            .bind(&body.table_number)
            .bind(&body.table_number)
            .bind(&body.food_name)
            .execute(&**db)
            .await;

        match response {
            Ok(id) => {
                HttpResponse::Created().json(DeleteOrder {
                    table_number: body.table_number.clone(),
                    food_name: body.food_name.clone(),
                    })
            },
            Err(_e) => HttpResponse::InternalServerError().json(OrderError{
                error: _e.to_string(),
            }),
        }
}

// TODO -> this should be parameterized url and just pass table_number e.g. /order/{table_number}
#[get("/order/list")]
pub async fn get_orders_by_table_num(db: Data<MySqlPool>, body: Json<ListOrder>) -> impl Responder {
     let response: Result<Vec<Order>, Error> = sqlx::query_as("SELECT * FROM `order` WHERE table_number = ?")
        .bind(&body.table_number)
        .fetch_all(&**db)
        .await;

        match response {
            Ok(res) => { HttpResponse::Created().json(res)
            },
            Err(_e) => HttpResponse::InternalServerError().json(OrderError{
                error: _e.to_string(),
                })
        }
}