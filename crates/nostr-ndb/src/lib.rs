// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! [`nostrdb`](https://github.com/damus-io/nostrdb) storage backend for Nostr apps

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![allow(clippy::mutable_key_type)] // TODO: remove when possible. Needed to suppress false positive for async_trait

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

pub extern crate nostr;
pub extern crate nostr_database as database;
pub extern crate nostrdb;

use nostr::secp256k1::schnorr::Signature;
use nostr_database::prelude::*;
use nostrdb::{Config, Filter as NdbFilter, Ndb, NdbStrVariant, Note, QueryResult, Transaction};

const MAX_RESULTS: i32 = 10_000;

// Wrap `Ndb` into `NdbDatabase` because only traits defined in the current crate can be implemented for types defined outside the crate!

/// [`nostrdb`](https://github.com/damus-io/nostrdb) backend
#[derive(Debug, Clone)]
pub struct NdbDatabase {
    db: Ndb,
}

impl NdbDatabase {
    /// Open nostrdb
    pub fn open<P>(path: P) -> Result<Self, DatabaseError>
    where
        P: AsRef<str>,
    {
        let path: &str = path.as_ref();
        let config = Config::new();

        Ok(Self {
            db: Ndb::new(path, &config).map_err(DatabaseError::backend)?,
        })
    }

    fn ndb_query<'a>(
        &self,
        txn: &'a Transaction,
        filters: Vec<Filter>,
    ) -> Result<Vec<QueryResult<'a>>, DatabaseError> {
        let filters = filters.into_iter().map(ndb_filter_conversion).collect();
        self.db
            .query(txn, filters, MAX_RESULTS)
            .map_err(DatabaseError::backend)
    }
}

impl Deref for NdbDatabase {
    type Target = Ndb;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl DerefMut for NdbDatabase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.db
    }
}

impl From<Ndb> for NdbDatabase {
    fn from(db: Ndb) -> Self {
        Self { db }
    }
}

#[async_trait]
impl NostrDatabase for NdbDatabase {
    fn backend(&self) -> Backend {
        Backend::LMDB
    }

    #[tracing::instrument(skip_all, level = "trace")]
    async fn save_event(&self, event: &Event) -> Result<bool, DatabaseError> {
        let msg = RelayMessage::event(SubscriptionId::new("ndb"), event.clone());
        let json: String = msg.as_json();
        self.db
            .process_event(&json)
            .map_err(DatabaseError::backend)?;
        Ok(true)
    }

    async fn check_id(&self, event_id: &EventId) -> Result<DatabaseEventStatus, DatabaseError> {
        let txn = Transaction::new(&self.db).map_err(DatabaseError::backend)?;
        let res = self.db.get_note_by_id(&txn, event_id.as_bytes());
        Ok(if res.is_ok() {
            DatabaseEventStatus::Saved
        } else {
            DatabaseEventStatus::NotExistent
        })
    }

    async fn has_coordinate_been_deleted(
        &self,
        _coordinate: &Coordinate,
        _timestamp: &Timestamp,
    ) -> Result<bool, DatabaseError> {
        Ok(false)
    }

    async fn event_id_seen(
        &self,
        _event_id: EventId,
        _relay_url: Url,
    ) -> std::result::Result<(), DatabaseError> {
        Ok(())
    }

    async fn event_seen_on_relays(
        &self,
        _event_id: &EventId,
    ) -> std::result::Result<Option<HashSet<Url>>, DatabaseError> {
        // TODO: use in-memory map to keep track of seen relays
        Err(DatabaseError::NotSupported)
    }

    #[tracing::instrument(skip_all, level = "trace")]
    async fn event_by_id(&self, event_id: &EventId) -> Result<Option<Event>, DatabaseError> {
        let txn = Transaction::new(&self.db).map_err(DatabaseError::backend)?;
        let note = self
            .db
            .get_note_by_id(&txn, event_id.as_bytes())
            .map_err(DatabaseError::backend)?;
        Ok(Some(ndb_note_to_event(note)?))
    }

    #[tracing::instrument(skip_all, level = "trace")]
    async fn count(&self, filters: Vec<Filter>) -> Result<usize, DatabaseError> {
        let txn: Transaction = Transaction::new(&self.db).map_err(DatabaseError::backend)?;
        let res: Vec<QueryResult> = self.ndb_query(&txn, filters)?;
        Ok(res.len())
    }

    #[tracing::instrument(skip_all, level = "trace")]
    async fn query(&self, filters: Vec<Filter>) -> Result<Events, DatabaseError> {
        let txn: Transaction = Transaction::new(&self.db).map_err(DatabaseError::backend)?;
        let mut events: Events = Events::new(&filters);
        let res: Vec<QueryResult> = self.ndb_query(&txn, filters)?;
        for r in res.into_iter() {
            events.insert(ndb_note_to_event(r.note)?);
        }
        Ok(events)
    }

    async fn negentropy_items(
        &self,
        filter: Filter,
    ) -> Result<Vec<(EventId, Timestamp)>, DatabaseError> {
        let txn: Transaction = Transaction::new(&self.db).map_err(DatabaseError::backend)?;
        let res: Vec<QueryResult> = self.ndb_query(&txn, vec![filter])?;
        Ok(res
            .into_iter()
            .map(|r| ndb_note_to_neg_item(r.note))
            .collect())
    }

    async fn delete(&self, _filter: Filter) -> Result<(), DatabaseError> {
        Err(DatabaseError::NotSupported)
    }

    async fn wipe(&self) -> Result<(), DatabaseError> {
        Err(DatabaseError::NotSupported)
    }
}

#[inline(always)]
fn ndb_filter_conversion(f: Filter) -> nostrdb::Filter {
    let mut filter = NdbFilter::new();

    if let Some(ids) = f.ids {
        if !ids.is_empty() {
            let ids: Vec<[u8; 32]> = ids.into_iter().map(|p| p.to_bytes()).collect();
            filter.ids(ids);
        }
    }

    if let Some(authors) = f.authors {
        if !authors.is_empty() {
            let authors: Vec<[u8; 32]> = authors.into_iter().map(|p| p.serialize()).collect();
            filter.authors(authors);
        }
    }

    if let Some(kinds) = f.kinds {
        if !kinds.is_empty() {
            let kinds: Vec<u64> = kinds.into_iter().map(|p| p.as_u16() as u64).collect();
            filter.kinds(kinds);
        }
    }

    if !f.generic_tags.is_empty() {
        for (single_letter, set) in f.generic_tags.into_iter() {
            filter.tags(set.into_iter().collect(), single_letter.as_char());
        }
    }

    if let Some(since) = f.since {
        filter.since(since.as_u64());
    }

    if let Some(until) = f.until {
        filter.until(until.as_u64());
    }

    if let Some(limit) = f.limit {
        filter.limit(limit as u64);
    }

    filter.build()
}

#[inline(always)]
fn ndb_note_to_event(note: Note) -> Result<Event, DatabaseError> {
    let id = EventId::from_byte_array(*note.id());
    let public_key = PublicKey::from_slice(note.pubkey()).map_err(DatabaseError::backend)?;
    let sig = Signature::from_slice(note.sig()).map_err(DatabaseError::backend)?;

    let tags: Vec<Tag> = ndb_note_to_tags(&note)?;

    let created_at = Timestamp::from(note.created_at());
    let kind = Kind::from(note.kind() as u16);
    let content = note.content();

    Ok(Event::new(
        id, public_key, created_at, kind, tags, content, sig,
    ))
}

#[inline(always)]
fn ndb_note_to_tags(note: &Note) -> Result<Vec<Tag>, DatabaseError> {
    let ndb_tags = note.tags();
    let mut tags: Vec<Tag> = Vec::with_capacity(ndb_tags.count() as usize);
    for tag in ndb_tags.iter() {
        let tag_str: Vec<String> = tag
            .into_iter()
            .map(|s| match s.variant() {
                NdbStrVariant::Id(id) => hex::encode(id),
                NdbStrVariant::Str(s) => s.to_owned(),
            })
            .collect();
        let tag: Tag = Tag::parse(&tag_str).map_err(DatabaseError::backend)?;
        tags.push(tag);
    }
    Ok(tags)
}

#[inline(always)]
fn ndb_note_to_neg_item(note: Note) -> (EventId, Timestamp) {
    let id = EventId::from_byte_array(*note.id());
    let created_at = Timestamp::from(note.created_at());
    (id, created_at)
}
