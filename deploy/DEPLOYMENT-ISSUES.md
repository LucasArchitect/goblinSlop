# GoblinSlop Deployment Issues & Fixes

> **Purpose**: Quick reference for common deployment problems found on production (GoblinTerra VPS).

---

## 1. DB Ownership Mismatch — "attempt to write a readonly database"

**Symptom**: Service starts but cannot write to SQLite DB. Journal shows:
```
Error: attempt to write a readonly database
```

**Cause**: `goblin_slop.db` file was created by a different user (e.g., `root` or `goblin`) from a previous manual deployment, while the systemd service runs as `User=goblinslop`.

**Fix**:
```bash
sudo chown goblinslop:goblinslop /home/goblinslop/goblin_slop.db
sudo systemctl restart goblinSlop
```

**Prevention**: The deploy script (`build-and-deploy.sh`) now explicitly checks and fixes DB ownership after extraction.

---

## 2. Stale Processes on Port 3000 — "Address in use"

**Symptom**: Service fails to start:
```
Failed to bind to 127.0.0.1:3000: Address in use
```

**Cause**: A previous `goblin_slop` process is still running (e.g., manually started, crashed without cleanup, or a rogue test instance).

**Fix**:
```bash
# Find the culprit
sudo lsof -ti :3000

# Kill it
sudo kill -9 <PID>

# Restart service
sudo systemctl restart goblinSlop
```

**Prevention**: The deploy script now kills stale non-service processes on port 3000 before starting.

---

## 3. Content Directory Mismatch — "Loading 0 content units"

**Symptom**: Service starts successfully but loads 0 articles. All pages show empty/fallback content.

**Cause**: `GOBLIN_CONTENT_DIR` environment variable points to old `/home/goblinslop/content/` (legacy `.md` format) instead of `/home/goblinslop/data/content/`.

**Fix**:
```bash
sudo sed -i 's|GOBLIN_CONTENT_DIR=.*content"|GOBLIN_CONTENT_DIR=/home/goblinslop/data/content"' /etc/systemd/system/goblinSlop.service
sudo systemctl daemon-reload && sudo systemctl restart goblinSlop
```

**Verification**:
```bash
journalctl -u goblinSlop --since "5 min ago" | grep "Loading content"
# Should show: ✅ Loaded content: <article-name> (slug: ..., date_added: ...)
```

---

## 4. Double Deploy Script Confusion

**Issue**: There were two deploy scripts (`build-and-deploy.sh` and `build-and-deploy-lukas.sh`) with slightly different SSH key handling, causing confusion about which one to use.

**Fix**: `build-and-deploy-lukas.sh` was removed. Use `build-and-deploy.sh` exclusively.

---

## Quick Deployment Checklist

Before running `build-and-deploy.sh`:

1. ✅ Local build succeeds: `cargo build --release`
2. ✅ Content files in `data/content/` (not legacy `content/`)
3. ✅ Images committed to git in `static/images/`
4. ✅ Service template has correct `__APP_USER__` placeholder substitution
5. ✅ No stale processes on port 3000: `ssh host 'sudo lsof -ti :3000'`

---

## Useful Debug Commands

```bash
# Check service status + recent logs
sudo systemctl status goblinSlop --no-pager
journalctl -u goblinSlop --since "5 min ago" --no-pager

# Verify content loading
journalctl -u goblinSlop --since "5 min ago" | grep -E "(Loading content|Loaded content)"

# Check DB ownership
ls -la /home/goblinslop/goblin_slop.db

# Check what's on port 3000
sudo ss -tlnp | grep 3000
sudo lsof -ti :3000

# Verify service file
cat /etc/systemd/system/goblinSlop.service | grep GOBLIN_CONTENT_DIR

# Quick health check (use GET, not HEAD)
curl -s --max-time 10 "http://127.0.0.1:3000/" | head -5
```
