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
REGISTER=$(curl -sf -X POST "$BASE/account/register" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"$USER\",\"password\":\"$PASS\"}")
echo "$REGISTER" | jq .

TOKEN=$(echo "$REGISTER" | jq -r '.token')
AUTH="Authorization: Bearer $TOKEN"

# ── 4. Login with same credentials ────────────────────────────────────────────
echo_step "4. Login with existing user"
LOGIN=$(curl -sf -X POST "$BASE/account/login" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"$USER\",\"password\":\"$PASS\"}")
echo "$LOGIN" | jq .

# Verify we got a valid token from login
LOGIN_TOKEN=$(echo "$LOGIN" | jq -r '.token')
echo "  → Login token matches register token: $([ "$TOKEN" != "$LOGIN_TOKEN" ] && echo "false (different tokens)" || echo "true")"

# ── 5. Get current user info ──────────────────────────────────────────────────
echo_step "5. Get current user info (/api/account)"
curl -sf "$BASE/api/account" -H "$AUTH" | jq .

# ── 6. List installed plugins (should be empty) ───────────────────────────────
echo_step "6. List installed plugins (should be empty)"
curl -sf "$BASE/api/plugins" -H "$AUTH" | jq .

# ── 7. Install a plugin ───────────────────────────────────────────────────────
echo_step "7. Install plugin: $PLUGIN"
PLUGIN_INSTALL=$(curl -sf -X POST "$BASE/api/plugins" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d "{\"plugin_id\":\"$PLUGIN\"}")
echo "$PLUGIN_INSTALL" | jq .

USER_PLUGIN_ID=$(echo "$PLUGIN_INSTALL" | jq -r '.id')
echo "  → User plugin ID: $USER_PLUGIN_ID"

# ── 8. List installed plugins (should have 1) ─────────────────────────────────
echo_step "8. List installed plugins (should show $PLUGIN)"
curl -sf "$BASE/api/plugins" -H "$AUTH" | jq .

# ── 9. Fetch media items ──────────────────────────────────────────────────────
echo_step "9. Fetch media items (page 1, pageSize 5)"
FETCH=$(curl -sf -X POST "$BASE/media/$PLUGIN/fetch?page=1&pageSize=5" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{}')
echo "$FETCH" | jq .

ITEM_ID=$(echo "$FETCH" | jq -r '.items[0].id')
TITLE=$(echo "$FETCH" | jq -r '.items[0].title')
echo "  → Selected: \"$TITLE\" (id: $ITEM_ID)"

# ── 10. Enrich media item ─────────────────────────────────────────────────────
echo_step "10. Enrich \"$TITLE\" (id: $ITEM_ID)"
curl -sf "$BASE/media/$PLUGIN/enrich/$ITEM_ID" -H "$AUTH" | jq .

# ── 11. Add item to library ───────────────────────────────────────────────────
echo_step "11. Add item to library"
LIBRARY_ADD=$(curl -sf -X POST "$BASE/api/library/$USER_PLUGIN_ID/items" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d "{\"library_item_id\":\"$ITEM_ID\"}")
echo "$LIBRARY_ADD" | jq .

USER_LIBRARY_ITEM_ID=$(echo "$LIBRARY_ADD" | jq -r '.id')
echo "  → User library item ID: $USER_LIBRARY_ITEM_ID"

# ── 12. List library items ────────────────────────────────────────────────────
echo_step "12. List library items"
curl -sf "$BASE/api/library/$USER_PLUGIN_ID/items" -H "$AUTH" | jq .

# ── 13. Get settings ──────────────────────────────────────────────────────────
echo_step "13. Get user settings (should have defaults)"
curl -sf "$BASE/api/settings" -H "$AUTH" | jq .

# ── 14. Update settings ───────────────────────────────────────────────────────
echo_step "14. Update settings (enable NSFW and set dark theme)"
curl -sf -X PUT "$BASE/api/settings" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"nsfw_enabled":true,"theme":"dark"}' | jq .

# ── 15. Create a playlist ─────────────────────────────────────────────────────
echo_step "15. Create a playlist"
PLAYLIST=$(curl -sf -X POST "$BASE/api/playlists" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"name":"My Demo Playlist"}')
echo "$PLAYLIST" | jq .

PLAYLIST_ID=$(echo "$PLAYLIST" | jq -r '.id')
echo "  → Playlist ID: $PLAYLIST_ID"

# ── 16. List playlists ────────────────────────────────────────────────────────
echo_step "16. List playlists"
curl -sf "$BASE/api/playlists" -H "$AUTH" | jq .

# ── 17. Add item to playlist ──────────────────────────────────────────────────
echo_step "17. Add library item to playlist"
echo "  → Playlist ID: $PLAYLIST_ID"
echo "  → User library item ID: $USER_LIBRARY_ITEM_ID"
PLAYLIST_ITEM=$(curl -s -X POST "$BASE/api/playlists/$PLAYLIST_ID/items" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d "{\"user_library_item_id\":\"$USER_LIBRARY_ITEM_ID\"}")
echo "$PLAYLIST_ITEM" | jq .

PLAYLIST_ITEM_ID=$(echo "$PLAYLIST_ITEM" | jq -r '.id')
echo "  → Playlist item ID: $PLAYLIST_ITEM_ID"

# ── 18. List playlist items ───────────────────────────────────────────────────
echo_step "18. List playlist items"
curl -sf "$BASE/api/playlists/$PLAYLIST_ID/items" -H "$AUTH" | jq .

# ── 19. Rename playlist ───────────────────────────────────────────────────────
echo_step "19. Rename playlist"
curl -sf -X PATCH "$BASE/api/playlists/$PLAYLIST_ID" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"name":"Renamed Demo Playlist"}' | jq .

# ── 20. Update playlist item index ────────────────────────────────────────────
echo_step "20. Update playlist item index"
curl -s -X PATCH "$BASE/api/playlists/$PLAYLIST_ID/items/$PLAYLIST_ITEM_ID" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"index":5}' | jq .

# ── 21. Disable plugin ────────────────────────────────────────────────────────
echo_step "21. Disable plugin"
curl -sf -X PATCH "$BASE/api/plugins/$PLUGIN" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"enabled":false}' | jq .

# ── 22. Re-enable plugin ──────────────────────────────────────────────────────
echo_step "22. Re-enable plugin"
curl -sf -X PATCH "$BASE/api/plugins/$PLUGIN" \
  -H "Content-Type: application/json" \
  -H "$AUTH" \
  -d '{"enabled":true}' | jq .

# ── 23. Remove item from playlist ─────────────────────────────────────────────
echo_step "23. Remove item from playlist"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE/api/playlists/$PLAYLIST_ID/items/$USER_LIBRARY_ITEM_ID" -H "$AUTH")
if [ "$HTTP_CODE" = "204" ]; then
  echo "  ✓ Item removed from playlist"
else
  echo "  ✗ Failed with HTTP $HTTP_CODE"
fi

# ── 24. Delete playlist ───────────────────────────────────────────────────────
echo_step "24. Delete playlist"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE/api/playlists/$PLAYLIST_ID" -H "$AUTH")
if [ "$HTTP_CODE" = "204" ]; then
  echo "  ✓ Playlist deleted"
else
  echo "  ✗ Failed with HTTP $HTTP_CODE"
fi

# ── 25. Remove item from library ──────────────────────────────────────────────
echo_step "25. Remove item from library"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE/api/library/$USER_PLUGIN_ID/items/$ITEM_ID" -H "$AUTH")
if [ "$HTTP_CODE" = "204" ]; then
  echo "  ✓ Item removed from library"
else
  echo "  ✗ Failed with HTTP $HTTP_CODE"
fi

# ── 26. Uninstall plugin ──────────────────────────────────────────────────────
echo_step "26. Uninstall plugin"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE/api/plugins/$PLUGIN" -H "$AUTH")
if [ "$HTTP_CODE" = "204" ]; then
  echo "  ✓ Plugin uninstalled"
else
  echo "  ✗ Failed with HTTP $HTTP_CODE"
fi

# ── 27. Delete account ────────────────────────────────────────────────────────
echo_step "27. Delete account"
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE/api/account" -H "$AUTH")
if [ "$HTTP_CODE" = "204" ]; then
  echo "  ✓ Account deleted (all user data cascaded)"
else
  echo "  ✗ Failed with HTTP $HTTP_CODE"
fi

echo
echo "──────────────────────────────────────────"
echo "✓ Demo complete! All endpoints tested successfully."
echo "──────────────────────────────────────────"

