
## Simple Restaurant jrbangit for Paidy

- Author: Ramoncito Jr Bangit
- email: jrbangit03@gmail.com

# How to test

LOCAL TESTING:
- run the following
    * `cargo build`
    * `cargo run`
    * `login to mysql using mysql -u root -p`
      * enter the password `root` in prompt
      * execute `create database simple_restaurant;`
      * execute `use simple_restaurant;`
      * execute the create table statements from `/sqls/tables.sql`
    * execute `sudo sh ./sqls/pre_populate_data.sh`
    * `test the endpoints using postman assuming the client caller is the waiter's ipad`
      * read and follow the endpoint descriptions below

TESTING THROUGH DOCKER:
- run the following
  * `docker compose build` - to make sure the image builds when we finally run docker-compose
  * `docker compose up simple_restaurant` - will automatically build the image
  * follow the same steps above for pre-population of data
  * `test the endpoints using postman  assuming the client caller is the waiter's ipad`
    * * read and follow the endpoint descriptions below

  
ENDPOINT DETAILS

- when a customer comes in, the waiter's ipad should call first
`/use_table` endpoint specifying the `table_number`. this will mark the table status to occupied and ready for accepting orders.
  * use postman or just direct curl
  * `curl -X POST -H "Content-Type: application/json" -d '{"table_number": 1}' "127.0.0.1:8080/use_table";`


- when viewing the menu, the waiter's ipad should call `/get_menu`
without any parameters
  * use postman or just direct curl
  * `curl 127.0.0.1:8080/get_menu`


- when adding new food to be available in the menu, the admin or waiter's ipad should call `/register_food`
specifying `food_name`, `cooking_time_in_mins`, `in_stock` as parameters
  * use postman or just direct curl
  * `curl -X POST -H "Content-Type: application/json" -d '{"food_name":"sushi", "cooking_time_in_mins":10, "in_stock": true}' "127.0.0.1:8080/register_food";`


- when adding orders, waiter's ipad should call `/order/create`
specifying  `table_number`, `food_id`, `food_name`, `quantity` in the request
  * use postman or just direct curl
  * `curl -X POST -H "Content-Type: application/json" -d '{"table_number": 2, "food_id": 1, "food_name":"sushi", "quantity": 2}' "127.0.0.1:8080/order/create";`
  * note: this requires the table_number status to be `OCCUPIED`. hence, call the `/use_table` endpoint first before creating an order.


- when deleting an order,  waiter's ipad should call `/order/delete`
specifying `food_name` and `table_number` in request
  * use postman or just direct curl
  * `curl -X POST -H "Content-Type: application/json" -d '{"table_number": 2, "food_name":"sushi"}' "127.0.0.1:8080/order/delete"; `


- when listing the current orders of a table_number, waiter's ipad should call `/order/list`
specifying `table_number` as parameter
  * use postman or just direct curl
  * `curl -X GET -H "Content-Type: application/json" -d '{"table_number": 2}' "127.0.0.1:8080/order/list";`


