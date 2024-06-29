DROP TABLE IF EXISTS players;

CREATE TABLE players (
    id INTEGER PRIMARY KEY,
    name TEXT CHECK(LENGTH(federation) <= 33),
    federation TEXT CHECK(LENGTH(federation) <= 3),
    sex INTEGER CHECK(sex <= 2),
    title INTEGER,
    fide_rating INTEGER CHECK(fide_rating <= 9999),
    fide_id TEXT
);
