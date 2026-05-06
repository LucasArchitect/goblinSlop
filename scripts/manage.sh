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
        echo -n "Home (port 80):   "
        curl -s -o /dev/null -w "%{http_code}\n" http://IP/
        echo -n "Home (port 3000): "
        curl -s -o /dev/null -w "%{http_code}\n" http://IP:3000/
        echo -n "Goblin Lore:     "
        curl -s -o /dev/null -w "%{http_code}\n" http://IP/goblin-lore
        echo -n "Dynamic page:    "
        curl -s -o /dev/null -w "%{http_code}\n" http://IP/some-test-path
        echo -n "API /api/all:    "
        curl -s -o /dev/null -w "%{http_code}\n" http://IP/api/all
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