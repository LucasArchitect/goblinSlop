use std::sync::Arc;
use tower_http::services::ServeDir;

mod config;
mod content;
mod data_loader;
mod db;
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

    // Load static content into database
    println!("Loading content from markdown files...");
    if let Err(e) = content::load_content_from_dir(&cfg.db_path, &cfg.content_dir) {
        eprintln!("Warning: Could not load all content: {}", e);
    }

    // Load scraped content into database
    println!("Loading scraped content...");
    if let Err(e) = data_loader::load_scraped_content(&cfg.db_path, &cfg.data_dir) {
        eprintln!("Warning: Could not load scraped content: {}", e);
    }

    // Build application state
    let state = routes::AppState { db };

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
    println!("🔮 Any URL will generate dynamic goblin content!");
    println!("📡 API: /api/all, /api/search?q=..., /api/content/:slug");
    println!("📝 Raw content: /raw/:slug");
    println!("⚙️  Config: host={} port={} db={} content={} static={} data={}",
        cfg.host, cfg.port, cfg.db_path, cfg.content_dir, cfg.static_dir, cfg.data_dir);

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
