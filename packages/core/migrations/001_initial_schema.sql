-- Migration 001: Initial schema
-- Stores individual fee data points collected from Horizon.
-- timestamps are stored as ISO 8601 / RFC 3339 strings.

CREATE TABLE IF NOT EXISTS fee_data_points (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    fee_amount       INTEGER NOT NULL,
    timestamp        TEXT    NOT NULL,
    transaction_hash TEXT    NOT NULL,
    ledger_sequence  INTEGER NOT NULL,
    created_at       TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_fee_data_points_timestamp
    ON fee_data_points (timestamp);

-- Stores periodic snapshots of Horizon fee_stats (base, min, max, avg).
-- Used by the historical API endpoint (Issue #13).

CREATE TABLE IF NOT EXISTS fee_snapshots (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    base_fee    TEXT NOT NULL,
    min_fee     TEXT NOT NULL,
    max_fee     TEXT NOT NULL,
    avg_fee     TEXT NOT NULL,
    captured_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_fee_snapshots_captured_at
    ON fee_snapshots (captured_at);