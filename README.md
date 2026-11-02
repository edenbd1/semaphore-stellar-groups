# Semaphore for Stellar — Trustless On-chain Groups

A **trustless** on-chain Semaphore group tree for Soroban/Stellar, companion to the
[semaphore-stellar](https://github.com/edenbd1/semaphore-stellar) library.

In the base library the Merkle tree is kept off-chain and the admin pushes roots (a trust
assumption). Here the **tree lives on-chain**: members are appended and the Merkle **root is
computed on-chain** from the actual membership, using a circomlib-compatible Poseidon (via
`soroban-poseidon` over CAP-75) and the @zk-kit **LeanIMT** rule. No one — not even the admin —
can publish a root that does not match the real members: the root is *derived*, not *asserted*.

The on-chain root is **identical** to what Semaphore's `Group` / @zk-kit `LeanIMT` computes, so
it interoperates with the sdk-29 verifier in the main library.

## Contract (`contracts/groups`)

- `create_group(admin)` → group id
- `add_member(group_id, commitment)` → new count (recomputes + emits the root on-chain)
- `root(group_id)` → current Merkle root
- `member_count(group_id)`

## Verified

```bash
cargo test            # on-chain Poseidon == circomlib; on-chain root == @zk-kit LeanIMT (sizes 1..5)
stellar contract build
```

Live on testnet (see `DEPLOYMENT.md`): adding members 1..5 yields the exact @zk-kit LeanIMT
root `11512324111804726054755717642058292259866309947044530224809882918003853859592`.

## Note on the toolchain

Built against **soroban-sdk 28** because the Poseidon host primitive is currently only reachable
through `soroban-poseidon` (sdk-28). The sdk-29 contracts in the other repos consume the root it
produces unchanged. Once the SDK exposes a public Poseidon (≥29), this can move to sdk 29.


## Status & safety

> ⚠️ **Prototype — unaudited, testnet only.** This code has **not** been audited and runs only
> on Stellar **testnet** with small fixtures. Do not use it with real value or on mainnet.
> Proofs reuse Ethereum's Semaphore trusted-setup ceremony (no new ceremony is performed).

## License

MIT — see [LICENSE](./LICENSE).
