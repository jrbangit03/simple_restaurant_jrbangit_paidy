use sqlx::{Pool, MySql, MySqlPool};


pub async fn database_connect() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://root:root@localhost:3306/simple_restaurant").await
}

