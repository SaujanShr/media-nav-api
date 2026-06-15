-- Indexes for common query patterns to improve performance

-- Users table
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);

-- User plugins table
CREATE INDEX IF NOT EXISTS idx_user_plugins_user_id ON user_plugins(user_id);
CREATE INDEX IF NOT EXISTS idx_user_plugins_plugin_id ON user_plugins(plugin_id);
CREATE INDEX IF NOT EXISTS idx_user_plugins_user_plugin ON user_plugins(user_id, plugin_id);

-- User library items table
CREATE INDEX IF NOT EXISTS idx_user_library_items_user_plugin_id ON user_library_items(user_plugin_id);
CREATE INDEX IF NOT EXISTS idx_user_library_items_library_item_id ON user_library_items(library_item_id);

-- User playlists table
CREATE INDEX IF NOT EXISTS idx_user_playlists_user_id ON user_playlists(user_id);

-- User playlist items table
CREATE INDEX IF NOT EXISTS idx_user_playlist_items_playlist_id ON user_playlist_items(playlist_id);
CREATE INDEX IF NOT EXISTS idx_user_playlist_items_user_library_item_id ON user_playlist_items(user_library_item_id);
CREATE INDEX IF NOT EXISTS idx_user_playlist_items_playlist_index ON user_playlist_items(playlist_id, index);
