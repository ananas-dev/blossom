DROP TABLE IF EXISTS players;

CREATE TABLE players (
    id INTEGER PRIMARY KEY,
    name TEXT,
    federation TEXT,
    sex INTEGER,
    title INTEGER,
    fide_rating INTEGER,
    fide_id TEXT
);
