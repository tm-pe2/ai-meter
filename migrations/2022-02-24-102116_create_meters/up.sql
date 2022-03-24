CREATE TABLE IF NOT EXISTS meters (
    id SERIAL PRIMARY KEY,
    occupants INT NOT NULL,
    day_consumption REAL NOT NULL,
    night_consumption REAL NOT NULL,
    last_snapshot TIMESTAMP NOT NULL
);
