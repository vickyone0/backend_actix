use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .max_lifetime(std::time::Duration::from_secs(30 * 60))
        .idle_timeout(std::time::Duration::from_secs(10 * 60))
        .test_before_acquire(true)
        .connect(database_url)
        .await
}
