#!/bin/bash
# Remote installation script - runs on the server
# This is copied as part of the deploy package and executed remotely.
set -e

echo "=== Installing GoblinSlop ==="

# Create application directory
mkdir -p /opt/goblinSlop

# Extract archive (tarball passed as first argument or read from /tmp)
TARBALL="${1:-/tmp/goblinSlop-deploy.tar.gz}"
if [ ! -f "$TARBALL" ]; then
    echo "Error: Tarball not found at $TARBALL"
    exit 1
fi
tar xzf "$TARBALL" -C /opt/goblinSlop/

# Make binary executable
chmod +x /opt/goblinSlop/goblin_slop

# Install systemd service
cp /opt/goblinSlop/goblinSlop.service /etc/systemd/system/goblinSlop.service
systemctl daemon-reload
systemctl enable goblinSlop
systemctl restart goblinSlop

# Install nginx config if nginx is present
if command -v nginx &> /dev/null; then
    echo "=== Installing nginx config ==="
    cp /opt/goblinSlop/goblinSlop.nginx /etc/nginx/sites-available/goblinSlop
    # Enable site if sites-enabled exists
    if [ -d /etc/nginx/sites-enabled ]; then
        ln -sf /etc/nginx/sites-available/goblinSlop /etc/nginx/sites-enabled/goblinSlop
    fi
    # Remove default site if present to avoid conflicts
    if [ -f /etc/nginx/sites-enabled/default ]; then
        rm -f /etc/nginx/sites-enabled/default
    fi
    nginx -t && systemctl reload nginx || echo "Warning: nginx config test failed"
fi

# Wait for service to be ready
sleep 3

# Verify
echo "=== Service Status ==="
systemctl status goblinSlop --no-pager || true

echo ""
echo "=== Health Check ==="
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/ || echo "000")
echo "HTTP Status from localhost:3000: ${HTTP_CODE}"

# Cleanup
rm -f "$TARBALL"

echo ""
echo "=== Installation Complete ==="
echo "Application: /opt/goblinSlop"
echo "Service:     goblinSlop (systemd)"
if command -v nginx &> /dev/null; then
    echo "Nginx:      /etc/nginx/sites-available/goblinSlop"
    echo "URL:        http://localhost:80 (via nginx)"
fi
echo "Backend:    http://localhost:3000 (direct)"