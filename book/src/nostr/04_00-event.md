# Event

## Serialize/deserialize to/from JSON

<custom-tabs category="lang">

<div slot="title">Rust</div>
<section>

```rust,ignore
{{#include ../../snippets/nostr/rust/src/event/json.rs}}
```

</section>

<div slot="title">Python</div>
<section>

```python,ignore
{{#include ../../snippets/nostr/python/src/event/json.py}}
```

</section>

<div slot="title">JavaScript</div>
<section>

```javascript,ignore
{{#include ../../snippets/nostr/js/src/event/json.js}}
```

</section>

<div slot="title">Kotlin</div>
<section>

```kotlin
{{#include ../../snippets/nostr/kotlin/shared/src/main/kotlin/rust/nostr/snippets/Event.kt:json}}
```

</section>

<div slot="title">Swift</div>
<section>

```swift
{{#include ../../snippets/nostr/swift/NostrSnippets/Sources/Event/Json.swift}}
```

</section>
</custom-tabs>

## Compose with event builder

A convenient way to compose events is by using the `EventBuilder`. It allow to compose `standard` and/or `custom` events.

<custom-tabs category="lang">

<div slot="title">Rust</div>
<section>

```rust,ignore
{{#include ../../snippets/nostr/rust/src/event/builder.rs}}
```

</section>

<div slot="title">Python</div>
<section>

```python,ignore
{{#include ../../snippets/nostr/python/src/event/builder.py}}
```

</section>

<div slot="title">JavaScript</div>
<section>

```javascript,ignore
{{#include ../../snippets/nostr/js/src/event/builder.js}}
```

</section>

<div slot="title">Kotlin</div>
<section>

```kotlin
{{#include ../../snippets/nostr/kotlin/shared/src/main/kotlin/rust/nostr/snippets/Event.kt:builder}}
```

</section>

<div slot="title">Swift</div>
<section>

```swift
{{#include ../../snippets/nostr/swift/NostrSnippets/Sources/Event/Builder.swift}}
```

</section>
</custom-tabs>