use sqlx::{Pool, MySql, MySqlPool};

// mysql://root:root@<container_name>/simple_restaurant
pub async fn database_connect() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://root:root@mysql/simple_restaurant").await
}

