// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::time::Duration;

use nostr_sdk::prelude::*;

const BECH32_SK: &str = "nsec12kcgs78l06p30jz7z7h3n2x2cy99nw2z6zspjdp7qc206887mwvs95lnkx";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let secret_key = SecretKey::from_bech32(BECH32_SK)?;
    let keys = Keys::new(secret_key);
    let opts = Options::new()
        .connection_timeout(Some(Duration::from_secs(10)))
        .send_timeout(Some(Duration::from_secs(5)));
    let client = Client::with_opts(&keys, opts);

    println!("Bot public key: {}", keys.public_key().to_bech32()?);

    client.add_relay("wss://nostr.oxtr.dev").await?;
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://nostr.mom").await?;
    client.add_relay("wss://nostr.wine").await?;
    client.add_relay("wss://relay.nostr.info").await?;
    client.add_relay("wss://auth.nostr1.com").await?;

    client.connect().await;

    let metadata = Metadata::new()
        .name("rust-nostr-bot-example")
        .display_name("rust-nostr bot example")
        .website(Url::parse("https://github.com/rust-nostr/nostr")?);
    client.set_metadata(&metadata).await?;

    let subscription = Filter::new()
        .pubkey(keys.public_key())
        .kind(Kind::GiftWrap)
        .limit(0); // Limit set to 0 to get only new events! Timestamp::now() CAN'T be used for gift wrap since the timestamps are tweaked!

    client.subscribe(vec![subscription], None).await?;

    client
        .handle_notifications(|notification| async {
            if let RelayPoolNotification::Event { event, .. } = notification {
                if event.kind == Kind::GiftWrap {
                    match client.unwrap_gift_wrap(&event).await {
                        Ok(UnwrappedGift { rumor, sender }) => {
                            if rumor.kind == Kind::PrivateDirectMessage {
                                let content: String = match rumor.content.as_str() {
                                    "/rand" => rand::random::<u16>().to_string(),
                                    "/help" => help(),
                                    _ => String::from(
                                        "Invalid command, send /help to see all commands.",
                                    ),
                                };

                                // Send private message to general relays
                                // client.send_private_msg(sender, content, None).await?;

                                // Send private message to specific relays
                                client
                                    .send_private_msg_to(
                                        ["wss://auth.nostr1.com"],
                                        sender,
                                        content,
                                        None,
                                    )
                                    .await?;
                            }
                        }
                        Err(e) => tracing::error!("Impossible to decrypt direct message: {e}"),
                    }
                }
            }
            Ok(false) // Set to true to exit from the loop
        })
        .await?;

    Ok(())
}

fn help() -> String {
    let mut output = String::new();
    output.push_str("Commands:\n");
    output.push_str("/rand - Random number\n");
    output.push_str("/help - Help");
    output
}
