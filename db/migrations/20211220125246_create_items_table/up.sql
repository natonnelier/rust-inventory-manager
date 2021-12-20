CREATE TABLE items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    user_id INTEGER,
    sell_price_cents INTEGER,
    buy_price_cents INTEGER,
    buy_date INTEGER,
    sell_date INTEGER
);
