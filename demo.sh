#!/usr/bin/env bash
# Full flow demo: hits all major API endpoints
# Prerequisites: server running (`make run`), database running, providers running
set -euo pipefail

BASE="http://127.0.0.1:8080"
USER="demo_user_$(date +%s)"
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

# ── 1. Health check ───────────────────────────────────────────────────────────
echo_step "1. Health check"
curl -sf "$BASE/health" | jq .

# ── 2. List all plugins (public) ──────────────────────────────────────────────
echo_step "2. List all available plugins (public)"
curl -sf "$BASE/plugins/all" | jq .

# ── 3. Register ───────────────────────────────────────────────────────────────
echo_step "3. Register user: $USER"
REGISTER=$(curl -sf -X POST "$BASE/auth/register" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"$USER\",\"password\":\"$PASS\"}")
echo "$REGISTER" | jq .

TOKEN=$(echo "$REGISTER" | jq -r '.token')
AUTH="Authorization: Bearer $TOKEN"

# ── 4. Get current user info ──────────────────────────────────────────────────
echo_step "4. Get current user info (/api/auth/me)"
curl -sf "$BASE/api/auth/me" -H "$AUTH" | jq .

# ── 5. List installed plugins (should be empty) ───────────────────────────────
echo_step "5. List installed plugins (should be empty)"
curl -sf "$BASE/api/plugins" -H "$AUTH" | jq .

# ── 6. Install a plugin ───────────────────────────────────────────────────────
echo_step "6. Install plugin: $PLUGIN"
PLUGIN_INSTALL=$(curl -sf -X POST "$BASE/api/plugins" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d "{\"plugin_id\":\"$PLUGIN\"}")
echo "$PLUGIN_INSTALL" | jq .

USER_PLUGIN_ID=$(echo "$PLUGIN_INSTALL" | jq -r '.id')
echo "  → User plugin ID: $USER_PLUGIN_ID"

# ── 7. List installed plugins (should have 1) ─────────────────────────────────
echo_step "7. List installed plugins (should show $PLUGIN)"
curl -sf "$BASE/api/plugins" -H "$AUTH" | jq .

# ── 8. Fetch media items ──────────────────────────────────────────────────────
echo_step "8. Fetch media items (page 1, pageSize 5)"
FETCH=$(curl -sf -X POST "$BASE/media/$PLUGIN/fetch?page=1&pageSize=5" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{}')
echo "$FETCH" | jq .

ITEM_ID=$(echo "$FETCH" | jq -r '.items[0].id')
TITLE=$(echo "$FETCH" | jq -r '.items[0].title')
echo "  → Selected: \"$TITLE\" (id: $ITEM_ID)"

# ── 9. Enrich media item ──────────────────────────────────────────────────────
echo_step "9. Enrich \"$TITLE\" (id: $ITEM_ID)"
curl -sf "$BASE/media/$PLUGIN/enrich/$ITEM_ID" -H "$AUTH" | jq .

# ── 10. Add item to library ───────────────────────────────────────────────────
echo_step "10. Add item to library"
LIBRARY_ADD=$(curl -sf -X POST "$BASE/api/library/$USER_PLUGIN_ID/items" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d "{\"library_item_id\":\"$ITEM_ID\"}")
echo "$LIBRARY_ADD" | jq .

USER_LIBRARY_ITEM_ID=$(echo "$LIBRARY_ADD" | jq -r '.id')
echo "  → User library item ID: $USER_LIBRARY_ITEM_ID"

# ── 11. List library items ────────────────────────────────────────────────────
echo_step "11. List library items"
curl -sf "$BASE/api/library/$USER_PLUGIN_ID/items" -H "$AUTH" | jq .

# ── 12. Get settings ──────────────────────────────────────────────────────────
echo_step "12. Get user settings (should have defaults)"
curl -sf "$BASE/api/settings" -H "$AUTH" | jq .

# ── 13. Update settings ───────────────────────────────────────────────────────
echo_step "13. Update settings (enable NSFW and set dark theme)"
curl -sf -X PUT "$BASE/api/settings" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"nsfw_enabled":true,"theme":"dark"}' | jq .

# ── 14. Create a playlist ─────────────────────────────────────────────────────
echo_step "14. Create a playlist"
PLAYLIST=$(curl -sf -X POST "$BASE/api/playlists" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"name":"My Demo Playlist"}')
echo "$PLAYLIST" | jq .

PLAYLIST_ID=$(echo "$PLAYLIST" | jq -r '.id')
echo "  → Playlist ID: $PLAYLIST_ID"

# ── 15. List playlists ────────────────────────────────────────────────────────
echo_step "15. List playlists"
curl -sf "$BASE/api/playlists" -H "$AUTH" | jq .

# ── 16. Add item to playlist ──────────────────────────────────────────────────
echo_step "16. Add library item to playlist"
PLAYLIST_ITEM=$(curl -sf -X POST "$BASE/api/playlists/$PLAYLIST_ID/items" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d "{\"user_library_item_id\":\"$USER_LIBRARY_ITEM_ID\"}")
echo "$PLAYLIST_ITEM" | jq .

PLAYLIST_ITEM_ID=$(echo "$PLAYLIST_ITEM" | jq -r '.id')
echo "  → Playlist item ID: $PLAYLIST_ITEM_ID"

# ── 17. List playlist items ───────────────────────────────────────────────────
echo_step "17. List playlist items"
curl -sf "$BASE/api/playlists/$PLAYLIST_ID/items" -H "$AUTH" | jq .

# ── 18. Rename playlist ───────────────────────────────────────────────────────
echo_step "18. Rename playlist"
curl -sf -X PATCH "$BASE/api/playlists/$PLAYLIST_ID" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"name":"Renamed Demo Playlist"}' | jq .

# ── 19. Update playlist item index ────────────────────────────────────────────
echo_step "19. Update playlist item index"
curl -sf -X PATCH "$BASE/api/playlists/$PLAYLIST_ID/items/$PLAYLIST_ITEM_ID" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"index":5.0}' | jq .

# ── 20. Disable plugin ────────────────────────────────────────────────────────
echo_step "20. Disable plugin"
curl -sf -X PATCH "$BASE/api/plugins/$PLUGIN" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"enabled":false}' | jq .

# ── 21. Re-enable plugin ──────────────────────────────────────────────────────
echo_step "21. Re-enable plugin"
curl -sf -X PATCH "$BASE/api/plugins/$PLUGIN" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"enabled":true}' | jq .

# ── 22. Remove item from playlist ─────────────────────────────────────────────
echo_step "22. Remove item from playlist"
curl -sf -X DELETE "$BASE/api/playlists/$PLAYLIST_ID/items/$USER_LIBRARY_ITEM_ID" \
  -H "$AUTH" | jq -c .

# ── 23. Delete playlist ───────────────────────────────────────────────────────
echo_step "23. Delete playlist"
curl -sf -X DELETE "$BASE/api/playlists/$PLAYLIST_ID" -H "$AUTH" | jq -c .

# ── 24. Remove item from library ──────────────────────────────────────────────
echo_step "24. Remove item from library"
curl -sf -X DELETE "$BASE/api/library/$USER_PLUGIN_ID/items/$ITEM_ID" \
  -H "$AUTH" | jq -c .

# ── 25. Uninstall plugin ──────────────────────────────────────────────────────
echo_step "25. Uninstall plugin"
curl -sf -X DELETE "$BASE/api/plugins/$PLUGIN" -H "$AUTH" | jq -c .

echo
echo "──────────────────────────────────────────"
echo "✓ Demo complete! All endpoints tested successfully."
echo "──────────────────────────────────────────"

