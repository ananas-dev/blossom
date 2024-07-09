DROP TABLE IF EXISTS players;
DROP TABLE IF EXISTS rounds;

CREATE TABLE players (
    id INTEGER PRIMARY KEY,
    name TEXT CHECK(LENGTH(federation) <= 33),
    federation TEXT CHECK(LENGTH(federation) <= 3),
    sex INTEGER CHECK(sex <= 2),
    title INTEGER,
    fide_rating INTEGER CHECK(fide_rating <= 9999),
    fide_id TEXT
);

CREATE TABLE rounds (
    id INTEGER PRIMARY KEY
);

CREATE TABLE games (
    id INTEGER PRIMARY KEY,
    round INTEGER,
    white INTEGER,
    black INTEGER,
    result INTEGER,
    FOREIGN KEY(round) REFERENCES rounds(id),
    FOREIGN KEY(white) REFERENCES players(id),
    FOREIGN KEY(black) REFERENCES players(id)
);
