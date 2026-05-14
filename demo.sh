#!/usr/bin/env bash
# Full flow: register → login → fetch anime → enrich → media
set -euo pipefail

BASE="http://127.0.0.1:8080"
USER="demo_user"
PASS="demo_pass"
PLUGIN="anime"

echo_step() { echo; echo "──────────────────────────────────────────"; echo "▶  $1"; echo "──────────────────────────────────────────"; }

# ── 1. Register ───────────────────────────────────────────────────────────────
echo_step "1. Register"
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

# ── 3. Fetch anime (page 1, 5 results) ───────────────────────────────────────
echo_step "3. Fetch anime (page 1, pageSize 5)"
FETCH=$(curl -sf -X POST "$BASE/media/$PLUGIN/fetch?page=1&pageSize=5" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{}')
echo "$FETCH" | jq .

# Pick the first item's id as our library_item_id (mal_id)
ITEM_ID=$(echo "$FETCH" | jq -r '.items[0].id')
TITLE=$(echo "$FETCH"  | jq -r '.items[0].title')
echo
echo "  → Selected: \"$TITLE\" (id: $ITEM_ID)"

# ── 4. Enrich ─────────────────────────────────────────────────────────────────
echo_step "4. Enrich \"$TITLE\" (id: $ITEM_ID)"
curl -sf "$BASE/media/$PLUGIN/enrich/$ITEM_ID" \
  -H "$AUTH" | jq .

# ── 5. Get media (episodes) ───────────────────────────────────────────────────
echo_step "5. Get media (episodes) for \"$TITLE\""
curl -sf "$BASE/media/$PLUGIN/library/$ITEM_ID/media" \
  -H "$AUTH" | jq .

