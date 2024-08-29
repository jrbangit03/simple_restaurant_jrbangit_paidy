#/bin/bash

for i in {1..10}; do
  curl -X POST -H "Content-Type: application/json" -d \
    '{"table_number":'$i', "table_status":"VACANT"}' "127.0.0.1:8080/add_table";
done;

curl -X POST -H "Content-Type: application/json" -d \
    '{"food_name":"sushi", "cooking_time_in_mins":10, "in_stock": true}' "127.0.0.1:8080/register_food";

curl -X POST -H "Content-Type: application/json" -d \
    '{"food_name":"pasta", "cooking_time_in_mins":15, "in_stock": true}' "127.0.0.1:8080/register_food";

curl -X POST -H "Content-Type: application/json" -d \
    '{"food_name":"chicken", "cooking_time_in_mins":5, "in_stock": true}' "127.0.0.1:8080/register_food";

curl -X POST -H "Content-Type: application/json" -d \
    '{"food_name":"burger", "cooking_time_in_mins":10, "in_stock": true}' "127.0.0.1:8080/register_food";

curl -X POST -H "Content-Type: application/json" -d \
    '{"food_name":"ramen", "cooking_time_in_mins":5, "in_stock": true}' "127.0.0.1:8080/register_food";

curl -X POST -H "Content-Type: application/json" -d \
    '{"food_name":"rice", "cooking_time_in_mins":5, "in_stock": true}' "127.0.0.1:8080/register_food";

curl -X POST -H "Content-Type: application/json" -d \
    '{"food_name":"beef", "cooking_time_in_mins":5, "in_stock": true}' "127.0.0.1:8080/register_food";