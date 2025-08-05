CREATE TABLE IF NOT EXISTS medicines (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL
);

INSERT INTO medicines (name, quantity) VALUES ('Paracetamol', 100), ('Ibuprofen', 50);
