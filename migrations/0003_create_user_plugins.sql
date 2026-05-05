CREATE TABLE IF NOT EXISTS user_plugins (
    id            TEXT        PRIMARY KEY,
    user_id       TEXT        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plugin_id     TEXT        NOT NULL REFERENCES plugins(id) ON DELETE CASCADE,
    last_accessed TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, plugin_id)
);
