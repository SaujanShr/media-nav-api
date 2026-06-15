#!/usr/bin/env bash
# Full flow demo: register → login → fetch → enrich → media
# Prerequisites: server running (`make run`), database running, providers running
set -euo pipefail

BASE="http://127.0.0.1:8080"
USER="demo_user"
PASS="demo_pass"
PLUGIN="example"

echo_step() { echo; echo "──────────────────────────────────────────"; echo "▶  $1"; echo "──────────────────────────────────────────"; }

# ── Prerequisites check ───────────────────────────────────────────────────────
echo_step "Checking prerequisites"
echo "  → Checking server at $BASE..."
if ! curl -sf "$BASE/health" > /dev/null 2>&1; then
  echo "❌ Server not running. Start it with: make run"
  exit 1
fi
echo "  ✓ Server is running"

echo "  → Checking database..."
if ! docker compose ps postgres | grep -q "Up"; then
  echo "❌ Database not running. Start it with: make db-start"
  exit 1
fi
echo "  ✓ Database is running"

echo "  → Checking providers..."
if ! curl -sf "http://localhost:4000/items" > /dev/null 2>&1; then
  echo "❌ Provider not running. Start it with: make start-providers"
  exit 1
fi
echo "  ✓ Providers are running"

# ── 1. Register ───────────────────────────────────────────────────────────────
echo_step "1. Register user"
curl -sf -X POST "$BASE/auth/register" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"$USER\",\"password\":\"$PASS\"}" | jq . || echo "(already registered, continuing)"

# ── 2. Login ──────────────────────────────────────────────────────────────────
echo_step "2. Login"
LOGIN=$(curl -sf -X POST "$BASE/auth/login" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"$USER\",\"password\":\"$PASS\"}")
echo "$LOGIN" | jq .

TOKEN=$(echo "$LOGIN" | jq -r '.token')
AUTH="Authorization: Bearer $TOKEN"

# ── 3. Fetch items ────────────────────────────────────────────────────────────
echo_step "3. Fetch media items (page 1, pageSize 5)"
FETCH=$(curl -sf -X POST "$BASE/media/$PLUGIN/fetch?page=1&pageSize=5" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{}')
echo "$FETCH" | jq .

# Pick the first item's id
ITEM_ID=$(echo "$FETCH" | jq -r '.items[0].id')
TITLE=$(echo "$FETCH"  | jq -r '.items[0].title')
echo
echo "  → Selected: \"$TITLE\" (id: $ITEM_ID)"

# ── 4. Enrich ─────────────────────────────────────────────────────────────────
echo_step "4. Enrich \"$TITLE\" (id: $ITEM_ID)"
curl -sf "$BASE/media/$PLUGIN/enrich/$ITEM_ID" \
  -H "$AUTH" | jq .

echo
echo "──────────────────────────────────────────"
echo "✓ Demo complete!"
echo "──────────────────────────────────────────"

