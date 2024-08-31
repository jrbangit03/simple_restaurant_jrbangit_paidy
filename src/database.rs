use sqlx::{Pool, MySql, MySqlPool};


pub async fn database_connect() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://root:root@0.0.0.0:3306/simple_restaurant").await
}

