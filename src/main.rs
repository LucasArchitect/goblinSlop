use std::sync::Arc;
use tower_http::services::ServeDir;

mod config;
mod db;
mod json_content_loader;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load configuration from environment variables
    let cfg = config::Config::from_env();

    println!("🧌 GoblinSlop starting with config: {:?}", cfg);

    // Initialize database
    let conn = db::init_db(&cfg.db_path).expect("Failed to initialize database");
    let db = Arc::new(std::sync::Mutex::new(conn));

    // Load all unified JSON content into database (single source of truth)
    println!("Loading content from unified JSON files...");
    if let Err(e) = json_content_loader::load_all_content(&cfg.db_path, &cfg.content_dir) {
        eprintln!("Warning: Could not load all content: {}", e);
    }

    // Build application state
    let state = routes::AppState { db, base_url: cfg.base_url.clone() };

    // Build router
    let app = routes::create_router(state)
        .nest_service("/static", ServeDir::new(&cfg.static_dir));

    // Start server
    let bind_addr = cfg.bind_addr();
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect(&format!("Failed to bind to {}", bind_addr));

    println!("🧌 GoblinSlop server running on http://{}", bind_addr);
    println!("📚 Content loaded. Browse to / for home page.");
    println!(
        "⚙️  Config: host={} port={} db={} content_dir={} static={}",
        cfg.host, cfg.port, cfg.db_path, cfg.content_dir, cfg.static_dir
    );

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
