// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use std::time::Duration;

use nostr_sdk::prelude::*;
use nostr_sdk_net::futures_util::{StreamExt};

const BECH32_SK: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let secret_key = SecretKey::from_bech32(BECH32_SK)?;
    let my_keys = Keys::new(secret_key);

    let client = Client::new(&my_keys);
    client.add_relay("wss://relay.nostr.info", None).await?;
    client.add_relay("wss://relay.damus.io", None).await?;

    client.connect().await;

    let relay = client.relays().await.get(&Url::parse("wss://relay.damus.io")?).unwrap();

    let filter = Filter::new()
        .pubkey(my_keys.public_key())
        .limit(100);

    let mut stream = relay.stream_events_of(vec![filter.clone()], Some(Duration::from_secs(10))).await?;

    while let Some(Ok(event)) = stream.next().await {
        println!("{event:?}");
    }

    Ok(())
}
