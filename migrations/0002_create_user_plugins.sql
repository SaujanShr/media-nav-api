CREATE TABLE IF NOT EXISTS user_plugins (
    id            TEXT        PRIMARY KEY,
    user_id       TEXT        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plugin_id     TEXT        NOT NULL,
    last_accessed TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    enabled       BOOLEAN     NOT NULL DEFAULT TRUE,
    UNIQUE (user_id, plugin_id)
);
