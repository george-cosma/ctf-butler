use std::{path::Path, sync::Arc};

use jsondb::{JsonDb, SchemaV0};
use serde::{Deserialize, Serialize};
use tokio::sync::OwnedRwLockWriteGuard;

pub struct State {
    db: Arc<JsonDb<SavedData>>,
}

impl State {
    pub const DATA_PATH: &'static str = "data.json";

    pub async fn new() -> Result<Self, anyhow::Error> {
        let path = Path::new(Self::DATA_PATH).to_path_buf();
        Ok(Self {
            db: Arc::new(JsonDb::load(path).await?),
        })
    }

    pub fn db_ref(&self) -> Arc<JsonDb<SavedData>> {
        self.db.clone()
    }

    pub async fn db(&self) -> tokio::sync::OwnedRwLockReadGuard<SavedData> {
        self.db.read().await
    }

    pub async fn db_mut(&self) -> OwnedRwLockWriteGuard<SavedData> {
        self.db.write().await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SavedData {
    pub test: String,
}

impl SchemaV0 for SavedData {}
