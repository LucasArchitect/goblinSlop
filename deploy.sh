#!/bin/bash
# Deploy GoblinSlop to production server
# Orchestrator: builds, packages, uploads, and installs remotely
set -e

REMOTE_HOST="IP"
REMOTE_USER="root"
SSH_PASSWORD="PASS"

echo "========================================"
echo "  GoblinSlop Deployment"
echo "  Target: ${REMOTE_USER}@${REMOTE_HOST}"
echo "========================================"

# Step 1: Build release binary
./scripts/build.sh

# Step 2: Package deployment files
./scripts/package.sh

# Step 3: Copy tarball to remote server
echo "=== Uploading to ${REMOTE_USER}@${REMOTE_HOST} ==="
sshpass -p "${SSH_PASSWORD}" scp -o StrictHostKeyChecking=no \
    goblinSlop-deploy.tar.gz \
    "${REMOTE_USER}@${REMOTE_HOST}:/tmp/"

# Step 4: Execute remote installation
echo "=== Running remote installation ==="
sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no \
    "${REMOTE_USER}@${REMOTE_HOST}" \
    "bash /opt/goblinSlop/install.sh /tmp/goblinSlop-deploy.tar.gz 2>/dev/null || \
     curl -s -o /tmp/install.sh https://raw.githubusercontent.com/.../scripts/install.sh 2>/dev/null; \
     test -f /tmp/install.sh && bash /tmp/install.sh /tmp/goblinSlop-deploy.tar.gz && rm -f /tmp/install.sh"

# If remote script doesn't exist, run inline installation
sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no \
    "${REMOTE_USER}@${REMOTE_HOST}" << 'REMOTEEOF'
set -e

echo "=== Remote installation ==="

# Create /opt/goblinSlop if needed
mkdir -p /opt/goblinSlop

# Extract archive
tar xzf /tmp/goblinSlop-deploy.tar.gz -C /opt/goblinSlop/

# Make binary executable
chmod +x /opt/goblinSlop/goblin_slop

# Copy install script to server for future use
cp /opt/goblinSlop/install.sh /opt/goblinSlop/install.sh 2>/dev/null || true

# Install systemd service
cp /opt/goblinSlop/goblinSlop.service /etc/systemd/system/goblinSlop.service
systemctl daemon-reload
systemctl enable goblinSlop
systemctl restart goblinSlop

# Install nginx config
echo "=== Setting up nginx ==="
apt-get install -y nginx 2>/dev/null || true

if command -v nginx &> /dev/null; then
    cp /opt/goblinSlop/goblinSlop.nginx /etc/nginx/sites-available/goblinSlop
    if [ -d /etc/nginx/sites-enabled ]; then
        ln -sf /etc/nginx/sites-available/goblinSlop /etc/nginx/sites-enabled/goblinSlop
        rm -f /etc/nginx/sites-enabled/default
    fi
    nginx -t && systemctl reload nginx || echo "Warning: nginx config test failed"
fi

sleep 3

# Verify
echo ""
echo "=== Verification ==="
systemctl status goblinSlop --no-pager | head -20
echo ""
echo -n "Backend (port 3000): "
curl -s -o /dev/null -w "HTTP %{http_code}\n" http://localhost:3000/ || echo "FAILED"
echo -n "Nginx (port 80):    "
curl -s -o /dev/null -w "HTTP %{http_code}\n" http://localhost:80/ || echo "FAILED"

# Cleanup
rm -f /tmp/goblinSlop-deploy.tar.gz

echo ""
echo "=== Deployment Complete ==="
echo "Backend: http://${REMOTE_HOST}:3000"
echo "Nginx:   http://${REMOTE_HOST}:80"
REMOTEEOF

# Cleanup local archive
rm -f goblinSlop-deploy.tar.gz

echo ""
echo "=== Local cleanup complete ==="