# Nostr Database

Database for Nostr apps

## Nostr Database Trait

This library contains the `NostrDatabase` and `NostrDatabaseExt` traits. You can use the [default backends](#default-backends) or implement your one (like PostgreSQL, ...).

## Default backends

* Memory (RAM), available in this library
* SQLite (desktop, server and mobile devices), available at [`nostr-sqlite`](https://crates.io/crates/nostr-sqlite)
* IndexedDB (web), available at [`nostr-indexeddb`](https://crates.io/crates/nostr-indexeddb)

## Crate Feature Flags

The following crate feature flags are available:

| Feature             | Default | Description                                                                              |
| ------------------- | :-----: | ---------------------------------------------------------------------------------------- |
| `flatbuf`           |   No    | Enable `flatbuffers` de/serialization for nostr events                                   |

## State

**This library is in an ALPHA state**, things that are implemented generally work but the API will change in breaking ways.

## License

This project is distributed under the MIT software license - see the [LICENSE](../../LICENSE) file for details

## Donations

⚡ Tips: <https://getalby.com/p/yuki>

⚡ Lightning Address: yuki@getalby.com
