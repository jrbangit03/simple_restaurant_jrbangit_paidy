
use actix_web::{Responder}

pub struct Food {
    food_id: i32;
    food_name: String;
}

pub struct Order {
    order_number: i32;
    table_number: i32;
    food: Vec<Food>
}

impl Order {
    pub fn new() -> Self {

    }
}

pub async fn create_new_order() -> impl Responder{

}