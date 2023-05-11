use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use axum_user_auth::{db, router, base, config};
use std::sync::Arc;
use axum::{
    Router, extract::Extension,
};



#[tokio::main]
async fn main() -> Result<(), base::errors::AppError> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_user_auth=debug");
    }

    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    db::axredis::init_redis_pool().await?;

    // 初始化mysql
    let database_url = std::env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set.");

    let pool = match MySqlPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await {
        Ok(pool) => {
            tracing::debug!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            tracing::debug!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_data = config::AppState{
        db_pool: pool,
    };

    let app = Router::new()
        .nest("/", router::routers())
        .layer(Extension(Arc::new(app_data)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    tracing::debug!("Serve running at http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
