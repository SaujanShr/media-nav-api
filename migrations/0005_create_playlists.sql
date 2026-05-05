CREATE TABLE IF NOT EXISTS playlists (
    id         TEXT        PRIMARY KEY,
    user_id    TEXT        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name       TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS playlist_items (
    id                   TEXT   PRIMARY KEY,
    playlist_id          TEXT   NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    user_library_item_id TEXT   NOT NULL REFERENCES user_library_items(id) ON DELETE CASCADE,
    index                FLOAT8 NOT NULL DEFAULT 0,
    UNIQUE (playlist_id, user_library_item_id)
);
