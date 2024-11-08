// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;

use nostr_sdk::prelude::*;
use wasm_bindgen::prelude::*;

pub mod filtering;
pub mod flags;
pub mod limits;
pub mod options;

use self::filtering::JsRelayFiltering;
use self::flags::JsAtomicRelayServiceFlags;
use self::options::{JsFilterOptions, JsRelayOptions, JsSubscribeOptions, JsSyncOptions};
use crate::database::JsEvents;
use crate::duration::JsDuration;
use crate::error::{into_err, Result};
use crate::protocol::event::{JsEvent, JsEventId};
use crate::protocol::message::JsClientMessage;
use crate::protocol::nips::nip11::JsRelayInformationDocument;
use crate::protocol::types::JsFilter;

#[derive(Clone)]
#[wasm_bindgen(js_name = Reconciliation)]
pub struct JsReconciliation {
    /// The IDs that were stored locally
    #[wasm_bindgen(getter_with_clone)]
    pub local: Vec<JsEventId>,
    /// The IDs that were missing locally (stored on relay)
    #[wasm_bindgen(getter_with_clone)]
    pub remote: Vec<JsEventId>,
    /// Events that are **successfully** sent to relays during reconciliation
    #[wasm_bindgen(getter_with_clone)]
    pub sent: Vec<JsEventId>,
    /// Event that are **successfully** received from relay
    #[wasm_bindgen(getter_with_clone)]
    pub received: Vec<JsEventId>,
    // TODO: add send_failures:
}

impl From<Reconciliation> for JsReconciliation {
    fn from(value: Reconciliation) -> Self {
        Self {
            local: value.local.into_iter().map(|e| e.into()).collect(),
            remote: value.remote.into_iter().map(|e| e.into()).collect(),
            sent: value.sent.into_iter().map(|e| e.into()).collect(),
            received: value.received.into_iter().map(|e| e.into()).collect(),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    /// Array
    #[wasm_bindgen(typescript_type = "JsRelay[]")]
    pub type JsRelayArray;
}

#[wasm_bindgen(js_name = Relay)]
pub struct JsRelay {
    inner: Relay,
}

impl From<Relay> for JsRelay {
    fn from(inner: Relay) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_name = RelayStatus)]
pub enum JsRelayStatus {
    /// Initialized
    Initialized,
    /// Pending
    Pending,
    /// Connecting
    Connecting,
    /// Connected
    Connected,
    /// Disconnected, will retry to connect again
    Disconnected,
    /// Completely disconnected
    Terminated,
}

impl From<RelayStatus> for JsRelayStatus {
    fn from(status: RelayStatus) -> Self {
        match status {
            RelayStatus::Initialized => Self::Initialized,
            RelayStatus::Pending => Self::Pending,
            RelayStatus::Connecting => Self::Connecting,
            RelayStatus::Connected => Self::Connected,
            RelayStatus::Disconnected => Self::Disconnected,
            RelayStatus::Terminated => Self::Terminated,
        }
    }
}

#[wasm_bindgen(js_class = Relay)]
impl JsRelay {
    /// Create new `Relay` with `in-memory` database
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str, opts: Option<JsRelayOptions>) -> Result<JsRelay> {
        let url: Url = Url::parse(url).map_err(into_err)?;
        let opts: RelayOptions = opts.map(|o| o.deref().clone()).unwrap_or_default();
        Ok(Self {
            inner: Relay::with_opts(url, opts),
        })
    }

    /// Get relay url
    pub fn url(&self) -> String {
        self.inner.url().to_string()
    }

    /// Get status
    pub fn status(&self) -> JsRelayStatus {
        self.inner.status().into()
    }

    /// Get Relay Service Flags
    pub fn flags(&self) -> JsAtomicRelayServiceFlags {
        self.inner.flags().clone().into()
    }

    /// Get relay filtering
    pub fn filtering(&self) -> JsRelayFiltering {
        self.inner.filtering().clone().into()
    }

    /// Check if relay is connected
    #[wasm_bindgen(js_name = isConnected)]
    pub fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    /// Get `RelayInformationDocument`
    pub async fn document(&self) -> JsRelayInformationDocument {
        self.inner.document().await.into()
    }

    // TODO: ad subscriptions

    // TODO: add subscription

    /// Get options
    pub fn opts(&self) -> JsRelayOptions {
        self.inner.opts().clone().into()
    }

    // TODO: add stats

    /// Get number of messages in queue
    pub fn queue(&self) -> u64 {
        self.inner.queue() as u64
    }

    /// Connect to relay and keep alive connection
    pub async fn connect(&self, connection_timeout: Option<JsDuration>) {
        self.inner.connect(connection_timeout.map(|d| *d)).await
    }

    /// Disconnect from relay and set status to 'Terminated'
    pub fn disconnect(&self) -> Result<()> {
        self.inner.disconnect().map_err(into_err)
    }

    /// Send msg to relay
    #[wasm_bindgen(js_name = sendMsg)]
    pub fn send_msg(&self, msg: &JsClientMessage) -> Result<()> {
        self.inner.send_msg(msg.deref().clone()).map_err(into_err)
    }

    /// Send multiple `ClientMessage` at once
    #[wasm_bindgen(js_name = batchMsg)]
    pub fn batch_msg(&self, msgs: Vec<JsClientMessage>) -> Result<()> {
        let msgs = msgs.into_iter().map(|msg| msg.deref().clone()).collect();
        self.inner.batch_msg(msgs).map_err(into_err)
    }

    /// Send event and wait for `OK` relay msg
    #[wasm_bindgen(js_name = sendEvent)]
    pub async fn send_event(&self, event: &JsEvent) -> Result<JsEventId> {
        Ok(self
            .inner
            .send_event(event.deref().clone())
            .await
            .map_err(into_err)?
            .into())
    }

    /// Send multiple `Event` at once
    #[wasm_bindgen(js_name = batchEvent)]
    pub async fn batch_event(&self, events: Vec<JsEvent>) -> Result<()> {
        let events = events.into_iter().map(|e| e.deref().clone()).collect();
        self.inner.batch_event(events).await.map_err(into_err)
    }

    /// Subscribe to filters
    ///
    /// ### Auto-closing subscription
    ///
    /// It's possible to automatically close a subscription by configuring the `SubscribeOptions`.
    pub async fn subscribe(
        &self,
        filters: Vec<JsFilter>,
        opts: &JsSubscribeOptions,
    ) -> Result<String> {
        let filters: Vec<Filter> = filters.into_iter().map(|f| f.into()).collect();
        Ok(self
            .inner
            .subscribe(filters, **opts) // TODO: allow to pass opts as reference
            .await
            .map_err(into_err)?
            .to_string())
    }

    /// Subscribe with custom subscription ID
    ///
    /// ### Auto-closing subscription
    ///
    /// It's possible to automatically close a subscription by configuring the `SubscribeOptions`.
    #[wasm_bindgen(js_name = subscribeWithId)]
    pub async fn subscribe_with_id(
        &self,
        id: &str,
        filters: Vec<JsFilter>,
        opts: &JsSubscribeOptions,
    ) -> Result<()> {
        let filters: Vec<Filter> = filters.into_iter().map(|f| f.into()).collect();
        self.inner
            .subscribe_with_id(SubscriptionId::new(id), filters, **opts) // TODO: allow to pass opts as reference
            .await
            .map_err(into_err)
    }

    /// Unsubscribe
    pub async fn unsubscribe(&self, id: String) -> Result<()> {
        self.inner
            .unsubscribe(SubscriptionId::new(id))
            .await
            .map_err(into_err)
    }

    /// Unsubscribe from all subscriptions
    #[wasm_bindgen(js_name = unsubscribeAll)]
    pub async fn unsubscribe_all(&self) -> Result<()> {
        self.inner.unsubscribe_all().await.map_err(into_err)
    }

    /// Fetch events
    #[wasm_bindgen(js_name = fetchEvents)]
    pub async fn fetch_events(
        &self,
        filters: Vec<JsFilter>,
        timeout: &JsDuration,
        opts: &JsFilterOptions,
    ) -> Result<JsEvents> {
        let filters: Vec<Filter> = filters.into_iter().map(|f| f.into()).collect();
        Ok(self
            .inner
            .fetch_events(filters, **timeout, **opts)
            .await
            .map_err(into_err)?
            .into())
    }

    /// Count events
    #[wasm_bindgen(js_name = countEvents)]
    pub async fn count_events(&self, filters: Vec<JsFilter>, timeout: &JsDuration) -> Result<u64> {
        let filters: Vec<Filter> = filters.into_iter().map(|f| f.into()).collect();
        Ok(self
            .inner
            .count_events(filters, **timeout)
            .await
            .map_err(into_err)? as u64)
    }

    /// Sync events with relay (negentropy reconciliation)
    pub async fn sync(&self, filter: &JsFilter, opts: &JsSyncOptions) -> Result<JsReconciliation> {
        self.inner
            .sync(filter.deref().clone(), opts.deref())
            .await
            .map_err(into_err)
            .map(|o| o.into())
    }
}
