use std::{env, path::Path, sync::Arc};

use anyhow::Context;
use libsql::{Builder, Connection, Database};

#[derive(Clone)]
pub struct Db {
    inner: Arc<Database>,
}

impl Db {
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let db = if database_url.starts_with("libsql://") || database_url.starts_with("https://") {
            let token = env::var("LIBSQL_AUTH_TOKEN")
                .context("LIBSQL_AUTH_TOKEN is required for remote libsql URL")?;
            Builder::new_remote(database_url.to_string(), token)
                .build()
                .await?
        } else {
            let path = parse_local_path(database_url);
            Builder::new_local(path).build().await?
        };

        Ok(Self {
            inner: Arc::new(db),
        })
    }

    pub fn connection(&self) -> anyhow::Result<Connection> {
        Ok(self.inner.connect()?)
    }
}

fn parse_local_path(database_url: &str) -> String {
    let raw = database_url
        .strip_prefix("file:")
        .unwrap_or(database_url)
        .trim();

    let path = if raw.is_empty() {
        "marketplace.db"
    } else {
        raw
    };

    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            let _ = std::fs::create_dir_all(parent);
        }
    }

    path.to_string()
}
