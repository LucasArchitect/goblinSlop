#!/bin/bash
# Package deployment files into a tarball
set -e

echo "=== Creating deployment package ==="
cd "$(dirname "$0")/.."

DEPLOY_DIR="deploy_package"
rm -rf "$DEPLOY_DIR"
mkdir -p "$DEPLOY_DIR"

# Copy binary
cp target/release/goblin_slop "$DEPLOY_DIR/"

# Copy static assets
cp -r static "$DEPLOY_DIR/"

# Copy content directory
cp -r content "$DEPLOY_DIR/"

# Copy data directory (for scraped_content.json)
cp -r data "$DEPLOY_DIR/"

# Copy existing SQLite database if it exists (to preserve dynamic pages)
if [ -f goblin_slop.db ]; then
    cp goblin_slop.db "$DEPLOY_DIR/"
fi

# Create systemd service file
cat > "$DEPLOY_DIR/goblinSlop.service" << 'SERVICEEOF'
[Unit]
Description=GoblinSlop - A chaotic collection of goblin knowledge
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/goblinSlop
ExecStart=/opt/goblinSlop/goblin_slop
Restart=always
RestartSec=5
Environment="RUST_LOG=info"
Environment="RUST_BACKTRACE=1"
Environment="GOBLIN_HOST=127.0.0.1"
Environment="GOBLIN_PORT=3000"
Environment="GOBLIN_DB_PATH=/opt/goblinSlop/goblin_slop.db"
Environment="GOBLIN_CONTENT_DIR=/opt/goblinSlop/content"
Environment="GOBLIN_STATIC_DIR=/opt/goblinSlop/static"
Environment="GOBLIN_DATA_DIR=/opt/goblinSlop/data"

[Install]
WantedBy=multi-user.target
SERVICEEOF

# Create nginx config
cat > "$DEPLOY_DIR/goblinSlop.nginx" << 'NGINXEOF'
server {
    listen 80;
    server_name _;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
NGINXEOF

# Package everything
tar czf goblinSlop-deploy.tar.gz -C "$DEPLOY_DIR" .
rm -rf "$DEPLOY_DIR"

echo "=== Package created: goblinSlop-deploy.tar.gz ==="