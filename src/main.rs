use actix_web::{web::Data, get, HttpServer, App, Responder};
// use serde::Serialize;

mod database;
use database::*;

mod routes;
use routes::order::*;


#[get("home")]
async fn home() -> impl Responder{
    let response: &str = "hey yo!";
    response
}

#[get("list_menu")]
async fn list_menu() -> impl Responder{
    let response: &str = "here's the menu";
    return response
}


// #[derive(Serialize)]
// struct Food {
//     food_number: i32
//     food_name: String
// }
//
// impl Food {
//     fn new (food_number: i32, food_name: String) -> Self {
//         Self{
//             food_number:food_number,
//             food_name:food_name
//             }
// }

// struct CustomerTable {
//     table_number: i32
// }
//
// struct Order {
// }
//
// struct Inventory {
// }


// since rust doesn't have runtime or doesn't have reflections, it cannot infer and find stuffs at runtime.
// hence, we need macros, to allow us to use async - tokio also has one.
#[tokio::main]
async fn main() -> std::io::Result<()> {

    let db = database_connect().await.expect("Failed to connect");

    println!("Connected to Database!");

    let server = HttpServer::new(move || {
        App::new()
        .app_data(Data::new(db.clone()))
        .service(home)
        .service(list_menu)
        .service(create_new_order)})
        .bind(("127.0.0.1", 8080))?
        .run();
    println!("server has started listening on port 8080!");
    server.await

}
