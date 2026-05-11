use std::env;

/// Application configuration loaded from environment variables.
///
/// # Environment Variables
///
/// | Variable | Default | Description |
/// |----------|---------|-------------|
/// | `GOBLIN_HOST` | `0.0.0.0` | Server bind address |
/// | `GOBLIN_PORT` | `3000` | Server port |
/// | `GOBLIN_DB_PATH` | `goblin_slop.db` | SQLite database file path |
/// | `GOBLIN_CONTENT_DIR` | `content` | Directory containing markdown files |
/// | `GOBLIN_STATIC_DIR` | `static` | Directory containing static assets |
/// | `GOBLIN_DATA_DIR` | `data` | Directory containing scraped data |
/// | `GOBLIN_BASE_URL` | `https://goblin.geno.su` | Base URL for canonical links & sitemap |
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_path: String,
    pub content_dir: String,
    pub static_dir: String,
    pub base_url: String,
}

impl Config {
    /// Load configuration from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        Config {
            host: env::var("GOBLIN_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("GOBLIN_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            db_path: env::var("GOBLIN_DB_PATH").unwrap_or_else(|_| "goblin_slop.db".to_string()),
            content_dir: env::var("GOBLIN_CONTENT_DIR").unwrap_or_else(|_| "data/content".to_string()),
            static_dir: env::var("GOBLIN_STATIC_DIR").unwrap_or_else(|_| "static".to_string()),
            base_url: env::var("GOBLIN_BASE_URL").unwrap_or_else(|_| "https://goblin.geno.su".to_string()),
        }
    }

    /// Return the formatted bind address (host:port).
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}