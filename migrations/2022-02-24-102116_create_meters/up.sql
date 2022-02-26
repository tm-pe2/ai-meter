CREATE TABLE IF NOT EXISTS meters (
    id SERIAL PRIMARY KEY,
    occupants INT NOT NULL,
    day_consumption REAL NOT NULL,
    night_consumtion REAL NOT NULL
);
