use std::sync::Arc;
use tower_http::services::ServeDir;

mod content;
mod db;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize database
    let conn = db::init_db("goblin_slop.db").expect("Failed to initialize database");
    let db = Arc::new(std::sync::Mutex::new(conn));

    // Load static content into database
    println!("Loading content from markdown files...");
    if let Err(e) = content::load_content_from_dir("goblin_slop.db", "content") {
        eprintln!("Warning: Could not load all content: {}", e);
    }

    // Build application state
    let state = routes::AppState { db };

    // Build router
    let app = routes::create_router(state)
        .nest_service("/static", ServeDir::new("static"));

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("🧌 GoblinSlop server running on http://0.0.0.0:3000");
    println!("📚 Content loaded. Browse to / for home page.");
    println!("🔮 Any URL will generate dynamic goblin content!");
    println!("📡 API: /api/all, /api/search?q=..., /api/content/:slug");
    println!("📝 Raw content: /raw/:slug");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}