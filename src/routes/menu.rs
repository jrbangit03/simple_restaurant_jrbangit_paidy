use actix_web::{Responder, web::{Data, Json}, post, get, HttpResponse};
use sqlx::{MySqlPool, FromRow, Error};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, FromRow)]
pub struct Food {
    food_id: i32,
    food_name: String,
    cooking_time_in_mins: i32,
    in_stock: bool,
}

// TODO -> NewFood should be impl as fn of Food struct
#[derive(Serialize, Deserialize)]
pub struct NewFood {
    food_name: String,
    cooking_time_in_mins: i32,
    in_stock: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FoodMenuError {
    error: String,
}

#[get("/get_menu")]
pub async fn get_menu(db: Data<MySqlPool>) -> impl Responder {
    let response: Result<Vec<Food>, Error> = sqlx::query_as("SELECT * FROM `food` WHERE in_stock = true")
    .fetch_all(&**db)
    .await;

    match response {
        Ok(res) => { HttpResponse::Created().json(res)
        },
        Err(_e) => HttpResponse::InternalServerError().json(FoodMenuError{
            error: _e.to_string(),
            })
    }
}


#[post("/register_food")]
pub async fn register_food(db: Data<MySqlPool>, body: Json<NewFood>) -> impl Responder {
    let response = sqlx::query("INSERT INTO `food` (`food_name`, `cooking_time_in_mins`, `in_stock`) VALUES (?, ?, ?)")
    .bind(&body.food_name)
    .bind(&body.cooking_time_in_mins)
    .bind(&body.in_stock)
    .execute(&**db)
    .await;

    match response {
        Ok(res) => {HttpResponse::Created().json(Food{
            food_id: res.last_insert_id() as i32,
            food_name: body.food_name.clone(),
            cooking_time_in_mins: body.cooking_time_in_mins.clone(),
            in_stock: body.in_stock.clone(),
            })
        },
        Err(_e) => HttpResponse::InternalServerError().json(FoodMenuError{
            error: _e.to_string(),
        })
    }
}
