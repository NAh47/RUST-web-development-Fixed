CREATE TABLE IF NOT EXISTS questions (
    id VARCHAR PRIMARY KEY,
    header VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    categories VARCHAR[]
);
