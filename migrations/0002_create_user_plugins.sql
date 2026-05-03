CREATE TABLE IF NOT EXISTS user_plugins (
    id         TEXT NOT NULL,
    user_id    TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plugin_id  TEXT NOT NULL,
    version_id TEXT NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (user_id, plugin_id)
);


