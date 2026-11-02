# Testnet deployment

Trustless on-chain group tree, verified live on Stellar **testnet**.

| | |
|---|---|
| `SemaphoreGroups` | `CBMLCUMADQ6H4SLHJAL7LMXJTAOEL74ZGLB5ZDFVL7NL4OHA5DZQ2YV3` |
| Deploy tx | [`30589a8b…`](https://stellar.expert/explorer/testnet/tx/30589a8bcc82aa009f03d70779e6e05cb51236550b5536ba0180269ec8eeaf73) |
| Flow | `create_group` → `add_member` × 5 (commitments 1..5) → `root` |
| On-chain root | `11512324111804726054755717642058292259866309947044530224809882918003853859592` |
| Matches | @zk-kit / Semaphore `Group` LeanIMT root for the same leaves — **computed on-chain** |

The root is derived on-chain from the actual members; the admin cannot forge it.
