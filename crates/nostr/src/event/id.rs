// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Event Id

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;

use bitcoin::hashes::sha256::Hash as Sha256Hash;
use bitcoin::hashes::Hash;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};

use super::{Kind, Tag};
use crate::nips::nip13;
use crate::nips::nip19::FromBech32;
use crate::nips::nip21::NostrURI;
use crate::util::hex;
use crate::{PublicKey, Timestamp};

/// Event ID size
pub const EVENT_ID_SIZE: usize = 32;

/// [`EventId`] error
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Hex decode error
    Hex(hex::Error),
    /// Invalid event ID
    InvalidEventId,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(e) => write!(f, "Hex: {e}"),
            Self::InvalidEventId => write!(f, "Invalid event ID"),
        }
    }
}

impl From<hex::Error> for Error {
    fn from(e: hex::Error) -> Self {
        Self::Hex(e)
    }
}

/// Event ID
///
/// 32-bytes lowercase hex-encoded sha256 of the serialized event data
///
/// <https://github.com/nostr-protocol/nips/blob/master/01.md>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId([u8; EVENT_ID_SIZE]);

impl EventId {
    /// Generate [`EventId`]
    pub fn new(
        public_key: &PublicKey,
        created_at: &Timestamp,
        kind: &Kind,
        tags: &[Tag],
        content: &str,
    ) -> Self {
        let json: Value = json!([0, public_key, created_at, kind, tags, content]);
        let event_str: String = json.to_string();
        let hash: Sha256Hash = Sha256Hash::hash(event_str.as_bytes());
        Self::owned(hash.to_byte_array())
    }

    /// Construct event ID
    #[inline]
    pub fn owned(bytes: [u8; EVENT_ID_SIZE]) -> Self {
        Self(bytes)
    }

    /// Try to parse [EventId] from `hex`, `bech32` or [NIP21](https://github.com/nostr-protocol/nips/blob/master/21.md) uri
    pub fn parse<S>(id: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let id: &str = id.as_ref();

        // Try from hex
        if let Ok(id) = Self::from_hex(id) {
            return Ok(id);
        }

        // Try from bech32
        if let Ok(id) = Self::from_bech32(id) {
            return Ok(id);
        }

        // Try from NIP21 URI
        if let Ok(id) = Self::from_nostr_uri(id) {
            return Ok(id);
        }

        Err(Error::InvalidEventId)
    }

    /// Parse from hex string
    #[inline]
    pub fn from_hex<S>(hex: S) -> Result<Self, Error>
    where
        S: AsRef<[u8]>,
    {
        let bytes: Vec<u8> = hex::decode(hex)?;
        Self::from_slice(&bytes)
    }

    /// Parse from bytes
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        // Check len
        if slice.len() != EVENT_ID_SIZE {
            return Err(Error::InvalidEventId);
        }

        // Copy bytes
        let mut bytes: [u8; EVENT_ID_SIZE] = [0u8; EVENT_ID_SIZE];
        bytes.copy_from_slice(slice);

        // Construct owned
        Ok(Self::owned(bytes))
    }

    /// All zeros
    #[inline]
    pub fn all_zeros() -> Self {
        Self::owned([0u8; EVENT_ID_SIZE])
    }

    /// Get as bytes
    #[inline]
    pub fn as_bytes(&self) -> &[u8; EVENT_ID_SIZE] {
        &self.0
    }

    /// Consume and get bytes
    #[inline]
    pub fn to_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Get as hex string
    #[inline]
    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    /// Check POW
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/13.md>
    #[inline]
    pub fn check_pow(&self, difficulty: u8) -> bool {
        nip13::get_leading_zero_bits(self.as_bytes()) >= difficulty
    }
}

impl FromStr for EventId {
    type Err = Error;

    /// Try to parse [EventId] from `hex` or `bech32`
    #[inline]
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Self::parse(id)
    }
}

impl AsRef<[u8; EVENT_ID_SIZE]> for EventId {
    fn as_ref(&self) -> &[u8; EVENT_ID_SIZE] {
        self.as_bytes()
    }
}

impl fmt::LowerHex for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl From<EventId> for Tag {
    fn from(event_id: EventId) -> Self {
        Tag::event(event_id)
    }
}

impl Serialize for EventId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for EventId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id: String = String::deserialize(deserializer)?;
        Self::parse(id).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pow() {
        let id =
            EventId::from_hex("2be17aa3031bdcb006f0fce80c146dea9c1c0268b0af2398bb673365c6444d45")
                .unwrap();
        assert!(!id.check_pow(16));

        // POW 20
        let id =
            EventId::from_hex("00000340cb60be5829fbf2712a285f12cf89e5db951c5303b731651f0d71ac1b")
                .unwrap();
        assert!(id.check_pow(16));
        assert!(id.check_pow(20));
        assert!(!id.check_pow(25));
    }
}
