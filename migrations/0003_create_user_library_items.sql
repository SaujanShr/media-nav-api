CREATE TABLE IF NOT EXISTS user_library_items (
    id              TEXT        PRIMARY KEY,
    user_plugin_id  TEXT        NOT NULL REFERENCES user_plugins(id) ON DELETE CASCADE,
    library_item_id TEXT        NOT NULL,
    last_accessed   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_plugin_id, library_item_id)
);
