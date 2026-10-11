use rusqlite::{Connection, Result};
use std::path::Path;

pub struct ProjectDatabase {
    conn: Connection,
}

impl ProjectDatabase {
    /// Opens or creates a single-file .lumina project container.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Ensure transactional safety, high concurrency, and low latency via WAL
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;",
        )?;

        let mut db = Self { conn };
        db.initialize_schema()?;

        Ok(db)
    }

    /// Creates the base relational tables for a Timeline
    fn initialize_schema(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;

        // Example schema structure supporting the Timeline Graph
        tx.execute(
            "CREATE TABLE IF NOT EXISTS timeline (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                timebase_value INTEGER NOT NULL,
                timebase_scale INTEGER NOT NULL
            )",
            [],
        )?;

        tx.execute(
            "CREATE TABLE IF NOT EXISTS video_track (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timeline_id TEXT NOT NULL,
                name TEXT NOT NULL,
                enabled BOOLEAN NOT NULL DEFAULT 1,
                locked BOOLEAN NOT NULL DEFAULT 0,
                FOREIGN KEY(timeline_id) REFERENCES timeline(id) ON DELETE CASCADE
            )",
            [],
        )?;

        tx.execute(
            "CREATE TABLE IF NOT EXISTS timeline_clip (
                id TEXT PRIMARY KEY,
                track_id INTEGER NOT NULL,
                asset_id TEXT NOT NULL,
                timeline_in_val INTEGER NOT NULL,
                timeline_in_scale INTEGER NOT NULL,
                timeline_out_val INTEGER NOT NULL,
                timeline_out_scale INTEGER NOT NULL,
                source_in_val INTEGER NOT NULL,
                source_in_scale INTEGER NOT NULL,
                source_out_val INTEGER NOT NULL,
                source_out_scale INTEGER NOT NULL,
                speed_multiplier REAL NOT NULL,
                FOREIGN KEY(track_id) REFERENCES video_track(id) ON DELETE CASCADE
            )",
            [],
        )?;

        tx.commit()?;
        Ok(())
    }
}
