CREATE TABLE IF NOT EXISTS meterdevices (
    id SERIAL PRIMARY KEY,
    meter SERIAL NOT NULL,
    device SERIAL NOT NULL,
    turned_on BOOL NOT NULL,
    duration INT,
    CONSTRAINT fk_meter FOREIGN KEY(meter) REFERENCES meters(id),
    CONSTRAINT fk_device FOREIGN KEY(device) REFERENCES devices(id)
);
