#!/bin/bash
# Build and deploy GoblinSlop to production server
# Reads DEPLOY_USER, DEPLOY_HOST, APP_USER from .env or environment
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$SCRIPT_DIR"

# Load .env if present
if [ -f .env ]; then
    set -a
    . .env
    set +a
fi

: "${DEPLOY_USER:?DEPLOY_USER not set — add to .env or export it}"
: "${DEPLOY_HOST:?DEPLOY_HOST not set — add to .env or export it}"
: "${APP_USER:?APP_USER not set — add to .env or export it}"
REMOTE="${DEPLOY_USER}@${DEPLOY_HOST}"

echo "========================================"
echo "  GoblinSlop Deployment"
echo "  Deploy as: ${REMOTE}"
echo "  App user:  ${APP_USER}"
echo "========================================"

# Step 1: Build release binary
echo "=== Building release binary ==="
cargo build --release
echo "=== Build complete ==="

# Step 2: Package deployment files (substituting APP_USER in service template)
echo "=== Creating deployment package ==="
DEPLOY_DIR="deploy_package"
rm -rf "$DEPLOY_DIR"
mkdir -p "$DEPLOY_DIR"

cp target/release/goblin_slop "$DEPLOY_DIR/"
cp -r static "$DEPLOY_DIR/"
cp -r data "$DEPLOY_DIR/"
sed "s/__APP_USER__/${APP_USER}/g" deploy/goblinSlop.service > "$DEPLOY_DIR/goblinSlop.service"

tar czf goblinSlop-deploy.tar.gz -C "$DEPLOY_DIR" .
rm -rf "$DEPLOY_DIR"
echo "=== Package created: goblinSlop-deploy.tar.gz ==="

# Step 3: Upload to remote server
echo "=== Uploading to ${REMOTE} ==="
scp goblinSlop-deploy.tar.gz "${REMOTE}:~/"
rm -f goblinSlop-deploy.tar.gz

# Step 4: Remote deploy — clean, extract, restart
echo "=== Running remote deployment ==="
ssh "${REMOTE}" APP_USER="${APP_USER}" bash << 'REMOTEEOF'
set -e

APP_USER_HOME="/home/${APP_USER}"

echo "Stopping service..."
sudo systemctl stop goblinSlop 2>/dev/null || echo "  (not running)"

echo "Cleaning old files from ${APP_USER_HOME}..."
sudo rm -rf "${APP_USER_HOME}/goblin_slop" \
            "${APP_USER_HOME}/static" \
            "${APP_USER_HOME}/data" \
            "${APP_USER_HOME}/content" \
            "${APP_USER_HOME}/goblin_slop.db" \
            "${APP_USER_HOME}/goblinSlop.service"

echo "Extracting fresh archive to ${APP_USER_HOME}..."
sudo tar xzf ~/goblinSlop-deploy.tar.gz -C "${APP_USER_HOME}/"
sudo chown -R "${APP_USER}:${APP_USER}" "${APP_USER_HOME}/"
sudo chmod +x "${APP_USER_HOME}/goblin_slop"
rm -f ~/goblinSlop-deploy.tar.gz

echo "Setting up systemd service..."
if [ ! -f /etc/systemd/system/goblinSlop.service ]; then
    sudo cp "${APP_USER_HOME}/goblinSlop.service" /etc/systemd/system/goblinSlop.service
    sudo systemctl daemon-reload
    sudo systemctl enable goblinSlop
    echo "Service created and enabled."
else
    sudo cp "${APP_USER_HOME}/goblinSlop.service" /etc/systemd/system/goblinSlop.service
    sudo systemctl daemon-reload
    echo "Service updated."
fi

echo "Starting service..."
sudo systemctl start goblinSlop

sleep 3

echo ""
echo "=== Verification ==="
systemctl is-active goblinSlop
systemctl status goblinSlop --no-pager | head -15
echo ""
echo -n "Backend (port 3000): "
curl -s -o /dev/null -w "HTTP %{http_code}\n" http://localhost:3000/ || echo "FAILED"

echo ""
echo "=== Deployment Complete ==="
REMOTEEOF