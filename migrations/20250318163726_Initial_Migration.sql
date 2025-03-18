CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (
      CAST(
        REPLACE(CAST(unixepoch('now') AS TEXT), '.', '')
        AS INTEGER
      )
    )
);


INSERT INTO products (name, price) VALUES ('Bread', 5.00);
INSERT INTO products (name, price) VALUES ('Water', 10.00);
