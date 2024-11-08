## Quickstart

<custom-tabs category="lang">

<div slot="title">Rust</div>
<section>

Create a client and connect to some relays.

```rust,ignore
{{#include ../../snippets/nostr-sdk/rust/src/quickstart.rs:create-client}}
```

Add metadata for the keys in the existing client.

```rust,ignore
{{#include ../../snippets/nostr-sdk/rust/src/quickstart.rs:create-metadata}}
```

Create a filter and notify the relays of the subscription.

```rust,ignore
{{#include ../../snippets/nostr-sdk/rust/src/quickstart.rs:create-filter}}
```

For more supported filters, view [the documentation](https://docs.rs/nostr-sdk/latest/nostr_sdk/struct.Filter.html).

Listen for notifications from the relays based on the subscribed filters and process them some way.

```rust, ignore
{{#include ../../snippets/nostr-sdk/rust/src/quickstart.rs:notifications}}
```

</section>

<div slot="title">Python</div>
<section>

Docs aren't ready yet, please check the examples at <https://github.com/rust-nostr/nostr/tree/master/bindings/nostr-sdk-ffi/python/examples>.

</section>

<div slot="title">JavaScript</div>
<section>

Docs aren't ready yet, please check the examples at <https://github.com/rust-nostr/nostr/tree/master/bindings/nostr-sdk-js/examples>.

</section>

<div slot="title">Kotlin</div>
<section>

TODO

</section>

<div slot="title">Swift</div>
<section>

TODO

</section>
</custom-tabs>
