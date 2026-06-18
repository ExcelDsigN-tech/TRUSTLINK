use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;
use trustlink_indexer::{api, config, db, poller};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = config::Config::from_env();

    let pool = db::create_pool(&cfg.database_url).await;
    db::run_migrations(&pool).await;

    let poller_instance = poller::EventPoller::new(cfg.clone(), pool.clone());
    tokio::spawn(async move {
        poller_instance.run().await;
    });

    let app = api::build_routes(pool).layer(CorsLayer::permissive());

    let addr = format!("{}:{}", cfg.host, cfg.port);
    tracing::info!("TrustLink indexer starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
