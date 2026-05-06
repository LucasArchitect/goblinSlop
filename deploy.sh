#!/bin/bash
# Deployment script for GoblinSlop
# Target: root@IP

set -e

REMOTE_HOST="IP"
REMOTE_USER="root"
REMOTE_DIR="/opt/goblinSlop"
SERVER_IP="IP"
SSH_PASSWORD="PASS"

echo "=== Creating deployment package ==="
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
Environment="GOBLIN_HOST=0.0.0.0"
Environment="GOBLIN_PORT=3000"
Environment="GOBLIN_DB_PATH=/opt/goblinSlop/goblin_slop.db"
Environment="GOBLIN_CONTENT_DIR=/opt/goblinSlop/content"
Environment="GOBLIN_STATIC_DIR=/opt/goblinSlop/static"
Environment="GOBLIN_DATA_DIR=/opt/goblinSlop/data"

[Install]
WantedBy=multi-user.target
SERVICEEOF

echo "=== Packaging archive ==="
tar czf goblinSlop-deploy.tar.gz -C "$DEPLOY_DIR" .
rm -rf "$DEPLOY_DIR"

echo "=== Deploying to ${REMOTE_USER}@${REMOTE_HOST} ==="

# Install sshpass if not available
if ! command -v sshpass &> /dev/null; then
    echo "Installing sshpass..."
    sudo apt-get install -y sshpass 2>/dev/null || true
fi

# Copy files to remote server
sshpass -p "${SSH_PASSWORD}" scp -o StrictHostKeyChecking=no goblinSlop-deploy.tar.gz "${REMOTE_USER}@${REMOTE_HOST}:/tmp/"

# Execute setup on remote server
sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${REMOTE_USER}@${REMOTE_HOST}" << 'REMOTEEOF'
set -e

echo "=== Setting up remote server ==="

# Create application directory
mkdir -p /opt/goblinSlop

# Extract archive
tar xzf /tmp/goblinSlop-deploy.tar.gz -C /opt/goblinSlop/

# Make binary executable
chmod +x /opt/goblinSlop/goblin_slop

# Install systemd service
cp /opt/goblinSlop/goblinSlop.service /etc/systemd/system/goblinSlop.service

# Reload systemd, enable and start service
systemctl daemon-reload
systemctl enable goblinSlop
systemctl restart goblinSlop

# Wait for service to be ready
sleep 3

# Check service status
echo "=== Service Status ==="
systemctl status goblinSlop --no-pager || true

# Cleanup
rm -f /tmp/goblinSlop-deploy.tar.gz

echo ""
echo "=== Checking if server is responding ==="
curl -s -o /dev/null -w "HTTP Status: %{http_code}\n" http://localhost:3000/ || echo "Server not yet ready, but service is installed."

echo ""
echo "=== Deployment Complete ==="
echo "Application installed at: /opt/goblinSlop"
echo "Service: goblinSlop"
echo "Server should be running on http://${SERVER_IP}:3000"
REMOTEEOF

# Cleanup local archive
rm -f goblinSlop-deploy.tar.gz

echo ""
echo "=== Local cleanup complete ==="