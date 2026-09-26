-- Add the tables
CREATE TABLE price_history
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    ticker    TEXT    NOT NULL,
    price     REAL    NOT NULL,
    polled_at INTEGER NOT NULL -- UTC seconds since epoch
);
CREATE INDEX idx_price_history_ticker_time ON price_history (ticker, polled_at);

CREATE TABLE candidate
(
    ticker         TEXT PRIMARY KEY,
    active         INTEGER NOT NULL DEFAULT 1 CHECK (active IN (0, 1)),
    deactivated_at INTEGER -- Nullable UTC seconds since epoch
);