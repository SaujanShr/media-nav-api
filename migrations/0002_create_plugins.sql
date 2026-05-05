CREATE TABLE IF NOT EXISTS plugins (
    id      TEXT    PRIMARY KEY,
    version TEXT    NOT NULL,
    nsfw    BOOLEAN NOT NULL
);
