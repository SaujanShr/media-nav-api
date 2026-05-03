CREATE TABLE IF NOT EXISTS user_library_items (
    id             TEXT NOT NULL,
    user_plugin_id TEXT NOT NULL REFERENCES user_plugins(id) ON DELETE CASCADE,
    library_item_id TEXT NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (user_plugin_id, library_item_id)
);

