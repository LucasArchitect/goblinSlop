#!/bin/bash
# Build and deploy GoblinSlop to goblinterra (Lukas variant)
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
SSH_KEY="${SSH_KEY:-~/.ssh/id_ed25519_goblin}"

echo "========================================"
echo "  GoblinSlop Deployment (Lukas variant)"
echo "  Deploy as: ${DEPLOY_USER}@${DEPLOY_HOST}"
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
# content/ directory removed — all content migrated to data/content/*.json
if [ -d "content" ]; then cp -r content "$DEPLOY_DIR/"; fi
cp -r data "$DEPLOY_DIR/"
sed "s/__APP_USER__/${APP_USER}/g" deploy/goblinSlop.service > "$DEPLOY_DIR/goblinSlop.service"

tar czf goblinSlop-deploy.tar.gz -C "$DEPLOY_DIR" .
rm -rf "$DEPLOY_DIR"
echo "=== Package created: goblinSlop-deploy.tar.gz ==="

# Step 3: Upload tarball to remote server
REMOTE="${DEPLOY_USER}@${DEPLOY_HOST}"
echo "=== Uploading to ${REMOTE} ==="
scp -i "$SSH_KEY" goblinSlop-deploy.tar.gz "${REMOTE}:~/goblinSlop-deploy.tar.gz"
rm -f goblinSlop-deploy.tar.gz

# Step 4: Create remote deploy script (heredoc with local variable expansion)
echo "=== Preparing remote deploy script ==="
TMPSCRIPT="/tmp/goblinSlop-deploy-remote.sh"
cat > "$TMPSCRIPT" << 'INNER_EOF'
#!/bin/bash
set -e
APP_USER="${1}"

echo "Extracting archive to /home/${APP_USER}/..."
sudo tar xzf ~/goblinSlop-deploy.tar.gz -C "/home/${APP_USER}/"
sudo chown -R "${APP_USER}:${APP_USER}" "/home/${APP_USER}/static" "/home/${APP_USER}/content" "/home/${APP_USER}/data" "/home/${APP_USER}/goblin_slop" 2>/dev/null || true
sudo chmod +x "/home/${APP_USER}/goblin_slop"
rm -f ~/goblinSlop-deploy.tar.gz

echo "Setting up systemd service..."
if [ ! -f /etc/systemd/system/goblinSlop.service ]; then
    sudo cp "/home/${APP_USER}/goblinSlop.service" /etc/systemd/system/goblinSlop.service
    sudo systemctl daemon-reload
    sudo systemctl enable goblinSlop
    echo "Service created and enabled."
else
    sudo cp "/home/${APP_USER}/goblinSlop.service" /etc/systemd/system/goblinSlop.service
    sudo systemctl daemon-reload
    echo "Service updated."
fi

echo "Restarting service..."
sudo systemctl restart goblinSlop

sleep 3

echo ""
echo "=== Verification ==="
systemctl status goblinSlop --no-pager | head -20
echo ""
HTTP_CODE=$(curl -s -o /dev/null -w '%{http_code}' http://localhost:3000/ 2>/dev/null) || HTTP_CODE="FAILED"
echo "Backend (port 3000): ${HTTP_CODE}"

echo ""
echo "=== Deployment Complete ==="
INNER_EOF

chmod +x "$TMPSCRIPT"

# Step 5: Upload deploy script and run it with APP_USER as argument
echo "=== Running remote deployment ==="
scp -i "$SSH_KEY" "$TMPSCRIPT" "${REMOTE}:/tmp/goblinSlop-deploy.sh"
ssh -i "$SSH_KEY" "${REMOTE}" bash /tmp/goblinSlop-deploy.sh "$APP_USER"
rm -f "/tmp/goblinSlop-deploy.sh" 2>/dev/null || true

rm -f "$TMPSCRIPT"

echo ""
echo "🧌 Deployment finished."
