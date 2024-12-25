use sqlx::PgPool;

pub async fn create_db_pool_sqlx() -> PgPool {
	PgPool::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL ENV VAR").as_str()).await.expect("couldnt connect")
}
