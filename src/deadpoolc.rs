use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

pub async fn create_pool() -> Pool {
    let mut cfg = Config::new();

    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);
    cfg.user = Some("user".to_string());
    cfg.password = Some("password".to_string());
    cfg.dbname = Some("dbname".to_string());

    cfg.pool = Some(deadpool_postgres::PoolConfig {
        max_size: 20,
        timeouts: deadpool_postgres::Timeouts {
            wait: Some(std::time::Duration::from_secs(5)),
            create: Some(std::time::Duration::from_secs(3)),
            recycle: Some(std::time::Duration::from_secs(30)),
        },
        ..Default::default()
    });

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(None, NoTls).unwrap()
}