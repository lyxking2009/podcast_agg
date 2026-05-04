use crate::Subscription;
use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};
use std::path::{Path, PathBuf};

pub fn default_library_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    PathBuf::from(home)
        .join("Library/Group Containers/243LU875E5.groups.com.apple.podcasts/Documents/MTLibrary.sqlite")
}

/// Read subscriptions from MTLibrary.sqlite. Copies the DB to a temp path first so we can open
/// it cleanly even while the Podcasts app holds the WAL.
pub fn read_subscriptions(library_path: &Path) -> Result<Vec<Subscription>> {
    let tmp = tempfile::Builder::new()
        .prefix("mtlibrary_")
        .suffix(".sqlite")
        .tempfile()
        .context("creating tempfile for SQLite copy")?;

    std::fs::copy(library_path, tmp.path())
        .with_context(|| format!("copying {} to temp", library_path.display()))?;

    let conn = Connection::open_with_flags(tmp.path(), OpenFlags::SQLITE_OPEN_READ_ONLY)
        .context("opening MTLibrary.sqlite (read-only)")?;

    let mut stmt = conn.prepare(
        "SELECT ZUUID, ZTITLE, ZAUTHOR, ZFEEDURL, ZWEBPAGEURL
         FROM ZMTPODCAST
         WHERE ZSUBSCRIBED = 1 AND ZFEEDURL IS NOT NULL
         ORDER BY ZTITLE COLLATE NOCASE",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Subscription {
            uuid: row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            title: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            author: row.get::<_, Option<String>>(2)?,
            feed_url: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
            web_url: row.get::<_, Option<String>>(4)?,
        })
    })?;

    let mut subs = Vec::new();
    for r in rows {
        let s = r?;
        if !s.feed_url.is_empty() && !s.title.is_empty() {
            subs.push(s);
        }
    }
    Ok(subs)
}

/// Read subscriptions from the Apple Podcasts SQLite library and write them as JSON
/// to `out`. Returns the number of subscriptions written.
pub fn refresh_to_file(library_path: &Path, out: &Path) -> Result<usize> {
    let subs = read_subscriptions(library_path)
        .with_context(|| format!("reading subscriptions from {}", library_path.display()))?;
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(out, serde_json::to_string_pretty(&subs)? + "\n")
        .with_context(|| format!("writing {}", out.display()))?;
    Ok(subs.len())
}
