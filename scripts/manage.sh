#!/bin/bash
# Server management utility
# Usage: ./scripts/manage.sh <command>
set -e

SSH_TARGET="root@IP"
SSH_PASSWORD="PASS"
SSHPASS_CMD="sshpass -p '${SSH_PASSWORD}' ssh -o StrictHostKeyChecking=no"

case "${1:-help}" in
    status)
        echo "=== Service Status ==="
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}" "systemctl status goblinSlop --no-pager"
        ;;
    logs)
        echo "=== Service Logs (follow) ==="
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}" "journalctl -u goblinSlop -f"
        ;;
    restart)
        echo "=== Restarting service ==="
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}" "systemctl restart goblinSlop"
        echo "Service restarted."
        ;;
    stop)
        echo "=== Stopping service ==="
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}" "systemctl stop goblinSlop"
        echo "Service stopped."
        ;;
    start)
        echo "=== Starting service ==="
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}" "systemctl start goblinSlop"
        echo "Service started."
        ;;
    nginx:restart)
        echo "=== Restarting nginx ==="
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}" "nginx -t && systemctl reload nginx"
        echo "Nginx reloaded."
        ;;
    ssh)
        echo "Connecting to ${SSH_TARGET}..."
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}"
        ;;
    check)
        echo "=== Checking endpoints ==="
        echo -n "HTTPS:           "
        curl -s -o /dev/null -w "%{http_code}\n" https://goblin.geno.su/
        echo -n "HTTP→HTTPS:      "
        curl -s -o /dev/null -w "%{http_code} → %{redirect_url}\n" http://goblin.geno.su/
        echo -n "Goblin Lore:     "
        curl -s -o /dev/null -w "%{http_code}\n" https://goblin.geno.su/goblin-lore
        echo -n "Dynamic page:    "
        curl -s -o /dev/null -w "%{http_code}\n" https://goblin.geno.su/some-test-path
        echo -n "API /api/all:    "
        curl -s -o /dev/null -w "%{http_code}\n" https://goblin.geno.su/api/all
        echo -n "Backend (127.0.0.1:3000): "
        curl -s -o /dev/null -w "%{http_code}\n" http://localhost:3000/
        ;;
    certbot:renew)
        echo "=== Manually renewing Let's Encrypt ==="
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}" "certbot renew 2>&1"
        echo "Renewal complete."
        ;;
    certbot:status)
        echo "=== Certificate Status ==="
        sshpass -p "${SSH_PASSWORD}" ssh -o StrictHostKeyChecking=no "${SSH_TARGET}" "certbot certificates 2>&1"
        ;;
    help|*)
        echo "GoblinSlop Server Management"
        echo ""
        echo "Usage: ./scripts/manage.sh <command>"
        echo ""
        echo "Commands:"
        echo "  status          Show systemd service status"
        echo "  logs            Follow service logs (Ctrl+C to exit)"
        echo "  restart         Restart the goblinSlop service"
        echo "  stop            Stop the goblinSlop service"
        echo "  start           Start the goblinSlop service"
        echo "  nginx:restart   Test and reload nginx config"
        echo "  ssh             SSH into the server"
        echo "  check           Run HTTP checks against all endpoints"
        echo "  certbot:renew   Manually renew Let's Encrypt certificate"
        echo "  certbot:status  Show certificate expiry info"
        echo "  help            Show this help message"
        ;;
    help|*)
        echo "GoblinSlop Server Management"
        echo ""
        echo "Usage: ./scripts/manage.sh <command>"
        echo ""
        echo "Commands:"
        echo "  status          Show systemd service status"
        echo "  logs            Follow service logs (Ctrl+C to exit)"
        echo "  restart         Restart the goblinSlop service"
        echo "  stop            Stop the goblinSlop service"
        echo "  start           Start the goblinSlop service"
        echo "  nginx:restart   Test and reload nginx config"
        echo "  ssh             SSH into the server"
        echo "  check           Run HTTP checks against all endpoints"
        echo "  help            Show this help message"
        ;;
esac