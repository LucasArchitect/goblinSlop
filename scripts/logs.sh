#!/bin/bash
# Nginx log viewer for GoblinSlop
# Usage: ./scripts/logs.sh [options]
set -e

SSH_TARGET="root@IP"
SSH_PASSWORD="PASS"
SSH_CMD="sshpass -p '${SSH_PASSWORD}' ssh -o StrictHostKeyChecking=no ${SSH_TARGET}"

show_help() {
    cat << 'EOF'
GoblinSlop Nginx Log Viewer

Usage: ./scripts/logs.sh [command]

Commands:
  tail       Show last 20 log lines (default)
  tail -n N  Show last N lines
  follow     Follow logs in real-time (Ctrl+C to exit)
  errors     Show recent error log entries
  top        Show top client IPs by request count
  urls       Show most requested URLs
  status     Show status code distribution
  agents     Show most common user agents
  today      Show today's request count
  slow       Show slowest requests (by response time)
  help       Show this help message

Examples:
  ./scripts/logs.sh tail -n 100
  ./scripts/logs.sh follow
  ./scripts/logs.sh top
  ./scripts/logs.sh slow
EOF
}

case "${1:-tail}" in
    tail)
        LINES="${2:-20}"
        echo "=== Last ${LINES} nginx access log entries ==="
        eval "${SSH_CMD} 'tail -n ${LINES} /var/log/nginx/access.log'"
        ;;
    follow)
        echo "=== Following nginx access log (Ctrl+C to stop) ==="
        eval "${SSH_CMD} 'tail -f /var/log/nginx/access.log'"
        ;;
    errors)
        echo "=== Recent error log entries ==="
        eval "${SSH_CMD} 'tail -n 30 /var/log/nginx/error.log'"
        ;;
    top)
        echo "=== Top client IPs ==="
        eval "${SSH_CMD} 'awk \"{print \\\$1}\" /var/log/nginx/access.log | sort | uniq -c | sort -rn | head -10'"
        ;;
    urls)
        echo "=== Most requested URLs ==="
        eval "${SSH_CMD} 'awk -F\\\" \"{print \\\$2}\" /var/log/nginx/access.log | awk \"{print \\\$2}\" | sort | uniq -c | sort -rn | head -10'"
        ;;
    status)
        echo "=== Status code distribution ==="
        eval "${SSH_CMD} 'awk \"{print \\\$(NF-2)}\" /var/log/nginx/access.log | sort | uniq -c | sort -rn'"
        ;;
    agents)
        echo "=== Most common user agents ==="
        eval "${SSH_CMD} 'awk -F\\\" \"{print \\\$6}\" /var/log/nginx/access.log | sort | uniq -c | sort -rn | head -10'"
        ;;
    today)
        TODAY=$(date +%d/%b/%Y)
        echo "=== Requests for $(date +%Y-%m-%d) ==="
        eval "${SSH_CMD} 'grep -c \"${TODAY}\" /var/log/nginx/access.log'"
        ;;
    slow)
        echo "=== Slowest requests (by upstream response time) ==="
        eval "${SSH_CMD} 'grep -oP \"rt=[0-9.]+ ms\" /var/log/nginx/access.log | sed \"s/rt=//;s/ ms//\" | paste - <(grep -oP \"GET [^ ]+\" /var/log/nginx/access.log) | sort -rn | head -10'"
        ;;
    help|*)
        show_help
        ;;
esac