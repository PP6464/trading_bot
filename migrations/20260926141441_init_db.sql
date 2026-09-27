-- All stocks that can be traded
CREATE TABLE stock
(
    ticker TEXT PRIMARY KEY
);

-- Where the screener is along the list of all stocks
CREATE TABLE screener_state
(
    id                  INTEGER PRIMARY KEY CHECK (id = 1), -- enforce single row
    last_ticker_checked TEXT
);

-- A candidate stock that should be tracked (if active)
CREATE TABLE candidate
(
    ticker         TEXT PRIMARY KEY REFERENCES stock (ticker),
    active         INTEGER NOT NULL DEFAULT 1 CHECK (active IN (0, 1)), -- Allows for soft deletion
    deactivated_at INTEGER                                              -- Nullable UTC seconds since epoch
);

-- An entry in the price history of a tracked stock
CREATE TABLE price_history
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    ticker    TEXT    NOT NULL REFERENCES candidate (ticker) ON DELETE CASCADE,
    price     REAL    NOT NULL,
    polled_at INTEGER NOT NULL -- UTC seconds since epoch
);
CREATE INDEX idx_price_history_ticker_time ON price_history (ticker, polled_at);