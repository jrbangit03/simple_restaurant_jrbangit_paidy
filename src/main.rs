use actix_web::{web::Data, get, HttpServer, App, Responder};

mod database;
use database::*;

mod routes;
use routes::order::*;
use routes::menu::*;
use routes::dine_in_tables::*;


#[get("home")]
async fn home() -> impl Responder{
    let response: &str = "welcome to jrbangit's simple restaurant!";
    response
}


// since rust doesn't have runtime or doesn't have reflections, it cannot infer and find stuffs at runtime.
// hence, we need macros, to allow us to use async - tokio also has one.
#[tokio::main]
async fn main() -> std::io::Result<()> {

    let db = database_connect().await.expect("Failed to connect");

    // change to logger
    println!("Connected to Database!");

    let server = HttpServer::new(move || {
        App::new()
        .app_data(Data::new(db.clone()))
        .service(home)
        .service(get_menu)
        .service(register_food)
        .service(create_new_order)
        .service(delete_order)
        .service(add_table)
        .service(use_table)
        .service(check_out_table)
        .service(get_orders_by_table_num)
        })
        .bind(("127.0.0.1", 8080))?
        .run();

     // change to logger
    println!("server has started listening on port 8080!");
    server.await

}
