//! Garbage collection.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use chrono::{Duration as ChronoDuration, Utc};
use futures::future::join_all;
use sea_orm::entity::prelude::*;
use sea_orm::query::QuerySelect;
use sea_orm::sea_query::{LockBehavior, LockType, Query};
use sea_orm::{ConnectionTrait, FromQueryResult};
use tokio::sync::Semaphore;
use tokio::time;
use tracing::instrument;

use super::{State, StateInner};
use crate::config::Config;
use crate::database::entity::cache::{self, Entity as Cache};
use crate::database::entity::chunk::{self, ChunkState, Entity as Chunk};
use crate::database::entity::chunkref::{self, Entity as ChunkRef};
use crate::database::entity::nar::{self, Entity as Nar, NarState};
use crate::database::entity::object::{self, Entity as Object};

#[derive(Debug, FromQueryResult)]
struct CacheIdAndRetentionPeriod {
    id: i64,
    name: String,
    retention_period: i32,
}

pub async fn list(config: Config) -> Result<()> {
    let state = StateInner::new(config).await;
    let db = state.database().await?;

    let caches = Cache::find()
        .all(db)
        .await?;

    for cache in caches {
        dbg!(cache);
    }

    let objects = Object::find()
        .all(db)
        .await?;

    for object in objects {
        dbg!(object);
    }

    Ok(())
}

