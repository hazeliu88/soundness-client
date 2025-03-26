# Soundness Layer

![Soundness Layer Banner](banner.png)

Soundness Layer is a decentralized verification layer that provides low latency, high throughput, and cross-chain compatibility for blockchain networks. Built on [Walrus](https://www.walrus.xyz/) for data availability and [Sui](https://sui.io/) for sequencing, it delivers a robust infrastructure for data verification. The network's security is maintained through an innovative restaking protocol.

[X](https://x.com/SoundnessLabs) | [Discord](https://discord.gg/F4cGbdqgw8) | [Telegram](https://t.me/SoundnessLabs) | [Website](https://soundness.xyz/)

> ⚠️ **Warning**: This is a testnet implementation. Do not use in production. The protocol is still under development and may contain bugs or security vulnerabilities. We are gradually rolling out features and open sourcing components as we progress through our development roadmap.

## Testnet Registration

We are currently preparing for testnet launch and invite early participants to register their keys. For detailed instructions on key generation and registration, please refer to the [`soundness-cli`](/soundness-cli) documentation.

## What has been Modified:

1. Ensure you have rust enviorment(cargo and rustc).
2. Jump to soundness-cli and do cargo run to get target:
```
cd soundness-cli
cargo run
```
3. Jump to target/debug and run the client:
```
cd target/debug
./soundness-cli generate-keys-bulk --count <COUNT>
```
4. Checkout File: key_store.json and key_store_with_mnemonic.json