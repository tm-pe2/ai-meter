CREATE TABLE IF NOT EXISTS devices (
    id SERIAL PRIMARY KEY,
    name VARCHAR UNIQUE NOT NULL,
    consumption REAL NOT NULL,
    duration INT
);
