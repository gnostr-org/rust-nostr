// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use nostr_sdk::database::DynNostrDatabase;
use nostr_sdk::{pool, FilterOptions, SubscriptionId, Url};
use uniffi::{Object, Record};

pub mod filtering;
pub mod limits;
pub mod options;
pub mod stats;
pub mod status;

pub use self::filtering::{RelayFiltering, RelayFilteringMode};
pub use self::limits::RelayLimits;
use self::options::SyncOptions;
pub use self::options::{ConnectionMode, RelayOptions, SubscribeOptions};
pub use self::stats::RelayConnectionStats;
pub use self::status::RelayStatus;
use crate::database::events::Events;
use crate::error::Result;
use crate::negentropy::NegentropyItem;
use crate::protocol::{ClientMessage, Event, EventId, Filter, RelayInformationDocument};
use crate::NostrDatabase;

#[derive(Record)]
pub struct ReconciliationSendFailureItem {
    pub id: Arc<EventId>,
    pub error: String,
}

/// Reconciliation output
#[derive(Record)]
pub struct Reconciliation {
    /// The IDs that were stored locally
    pub local: Vec<Arc<EventId>>,
    /// The IDs that were missing locally (stored on relay)
    pub remote: Vec<Arc<EventId>>,
    /// Events that are **successfully** sent to relays during reconciliation
    pub sent: Vec<Arc<EventId>>,
    /// Event that are **successfully** received from relay
    pub received: Vec<Arc<EventId>>,

    pub send_failures: HashMap<String, Vec<ReconciliationSendFailureItem>>,
}

impl From<pool::Reconciliation> for Reconciliation {
    fn from(value: pool::Reconciliation) -> Self {
        Self {
            local: value
                .local
                .into_iter()
                .map(|e| Arc::new(e.into()))
                .collect(),
            remote: value
                .remote
                .into_iter()
                .map(|e| Arc::new(e.into()))
                .collect(),
            sent: value.sent.into_iter().map(|e| Arc::new(e.into())).collect(),
            received: value
                .received
                .into_iter()
                .map(|e| Arc::new(e.into()))
                .collect(),
            send_failures: value
                .send_failures
                .into_iter()
                .map(|(url, map)| {
                    (
                        url.to_string(),
                        map.into_iter()
                            .map(|(id, e)| ReconciliationSendFailureItem {
                                id: Arc::new(id.into()),
                                error: e,
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Object)]
pub struct Relay {
    inner: pool::Relay,
}

impl From<pool::Relay> for Relay {
    fn from(inner: pool::Relay) -> Self {
        Self { inner }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Relay {
    /// Create new `Relay` with **default** `options` and `in-memory database`
    #[uniffi::constructor]
    pub fn new(url: &str) -> Result<Self> {
        let url: Url = Url::parse(url)?;
        Ok(Self {
            inner: nostr_sdk::Relay::new(url),
        })
    }

    /// Create new `Relay` with default `in-memory database` and custom `options`
    #[uniffi::constructor]
    pub fn with_opts(url: &str, opts: &RelayOptions) -> Result<Self> {
        let url: Url = Url::parse(url)?;
        let opts = opts.deref().clone();
        Ok(Self {
            inner: nostr_sdk::Relay::with_opts(url, opts),
        })
    }

    /// Create new `Relay` with **custom** `database` and/or `options`
    #[uniffi::constructor]
    pub fn custom(url: String, database: &NostrDatabase, opts: &RelayOptions) -> Result<Self> {
        let url: Url = Url::parse(&url)?;
        let database: Arc<DynNostrDatabase> = database.into();
        let opts = opts.deref().clone();
        Ok(Self {
            inner: nostr_sdk::Relay::custom(url, database, opts),
        })
    }

    /// Get relay url
    pub fn url(&self) -> String {
        self.inner.url().to_string()
    }

    /// Get connection mode
    pub fn connection_mode(&self) -> ConnectionMode {
        self.inner.connection_mode().clone().into()
    }

    /// Get status
    pub fn status(&self) -> RelayStatus {
        self.inner.status().into()
    }

    /* /// Get Relay Service Flags
    pub fn flags(&self) -> AtomicRelayServiceFlags {
        self.inner.flags()
    } */

    /// Get relay filtering
    pub fn filtering(&self) -> RelayFiltering {
        self.inner.filtering().clone().into()
    }

    /// Check if `Relay` is connected
    pub fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    pub async fn document(&self) -> Arc<RelayInformationDocument> {
        Arc::new(self.inner.document().await.into())
    }

    pub async fn subscriptions(&self) -> HashMap<String, Vec<Arc<Filter>>> {
        self.inner
            .subscriptions()
            .await
            .into_iter()
            .map(|(id, filters)| {
                (
                    id.to_string(),
                    filters.into_iter().map(|f| Arc::new(f.into())).collect(),
                )
            })
            .collect()
    }

    /// Get filters by subscription ID
    pub async fn subscription(&self, id: String) -> Option<Vec<Arc<Filter>>> {
        let id = SubscriptionId::new(id);
        self.inner
            .subscription(&id)
            .await
            .map(|f| f.into_iter().map(|f| Arc::new(f.into())).collect())
    }

    pub fn opts(&self) -> RelayOptions {
        self.inner.opts().clone().into()
    }

    pub fn stats(&self) -> Arc<RelayConnectionStats> {
        Arc::new(self.inner.stats().clone().into())
    }

    /// Get number of messages in queue
    pub fn queue(&self) -> u64 {
        self.inner.queue() as u64
    }

    // TODO: add notifications

    /// Connect to relay and keep alive connection
    pub async fn connect(&self, connection_timeout: Option<Duration>) {
        self.inner.connect(connection_timeout).await
    }

    /// Disconnect from relay and set status to 'Terminated'
    pub fn disconnect(&self) -> Result<()> {
        Ok(self.inner.disconnect()?)
    }

    /// Send msg to relay
    pub fn send_msg(&self, msg: Arc<ClientMessage>) -> Result<()> {
        Ok(self.inner.send_msg(msg.as_ref().deref().clone())?)
    }

    /// Send multiple `ClientMessage` at once
    pub fn batch_msg(&self, msgs: Vec<Arc<ClientMessage>>) -> Result<()> {
        let msgs = msgs
            .into_iter()
            .map(|msg| msg.as_ref().deref().clone())
            .collect();
        Ok(self.inner.batch_msg(msgs)?)
    }

    /// Send event and wait for `OK` relay msg
    pub async fn send_event(&self, event: &Event) -> Result<Arc<EventId>> {
        Ok(Arc::new(
            self.inner.send_event(event.deref().clone()).await?.into(),
        ))
    }

    /// Send multiple `Event` at once
    pub async fn batch_event(&self, events: Vec<Arc<Event>>) -> Result<()> {
        let events = events
            .into_iter()
            .map(|e| e.as_ref().deref().clone())
            .collect();
        Ok(self.inner.batch_event(events).await?)
    }

    /// Subscribe to filters
    ///
    /// Internally generate a new random subscription ID. Check `subscribe_with_id` method to use a custom subscription ID.
    ///
    /// ### Auto-closing subscription
    ///
    /// It's possible to automatically close a subscription by configuring the `SubscribeOptions`.
    ///
    /// Note: auto-closing subscriptions aren't saved in subscriptions map!
    pub async fn subscribe(
        &self,
        filters: Vec<Arc<Filter>>,
        opts: &SubscribeOptions,
    ) -> Result<String> {
        Ok(self
            .inner
            .subscribe(
                filters
                    .into_iter()
                    .map(|f| f.as_ref().deref().clone())
                    .collect(),
                **opts,
            )
            .await?
            .to_string())
    }

    /// Subscribe with custom subscription ID
    ///
    /// ### Auto-closing subscription
    ///
    /// It's possible to automatically close a subscription by configuring the `SubscribeOptions`.
    ///
    /// Note: auto-closing subscriptions aren't saved in subscriptions map!
    pub async fn subscribe_with_id(
        &self,
        id: String,
        filters: Vec<Arc<Filter>>,
        opts: &SubscribeOptions,
    ) -> Result<()> {
        Ok(self
            .inner
            .subscribe_with_id(
                SubscriptionId::new(id),
                filters
                    .into_iter()
                    .map(|f| f.as_ref().deref().clone())
                    .collect(),
                **opts,
            )
            .await?)
    }

    /// Unsubscribe
    pub async fn unsubscribe(&self, id: String) -> Result<()> {
        Ok(self.inner.unsubscribe(SubscriptionId::new(id)).await?)
    }

    /// Unsubscribe from all subscriptions
    pub async fn unsubscribe_all(&self) -> Result<()> {
        Ok(self.inner.unsubscribe_all().await?)
    }

    /// Fetch events
    pub async fn fetch_events(
        &self,
        filters: Vec<Arc<Filter>>,
        timeout: Duration,
    ) -> Result<Events> {
        let filters = filters
            .into_iter()
            .map(|f| f.as_ref().deref().clone())
            .collect();
        Ok(self
            .inner
            .fetch_events(filters, timeout, FilterOptions::ExitOnEOSE)
            .await?
            .into())
    }

    /// Count events
    pub async fn count_events(&self, filters: Vec<Arc<Filter>>, timeout: Duration) -> Result<u64> {
        let filters = filters
            .into_iter()
            .map(|f| f.as_ref().deref().clone())
            .collect();
        Ok(self.inner.count_events(filters, timeout).await? as u64)
    }

    /// Sync events with relays (negentropy reconciliation)
    pub async fn sync(&self, filter: &Filter, opts: &SyncOptions) -> Result<Reconciliation> {
        Ok(self
            .inner
            .sync(filter.deref().clone(), opts.deref())
            .await?
            .into())
    }

    /// Sync events with relays (negentropy reconciliation)
    pub async fn sync_with_items(
        &self,
        filter: &Filter,
        items: Vec<NegentropyItem>,
        opts: &SyncOptions,
    ) -> Result<Reconciliation> {
        let items = items
            .into_iter()
            .map(|item| (**item.id, **item.timestamp))
            .collect();
        Ok(self
            .inner
            .sync_with_items(filter.deref().clone(), items, opts.deref())
            .await?
            .into())
    }
}
