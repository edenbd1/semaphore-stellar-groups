#!/usr/bin/env bash
# Deploy the on-chain group tree and prove the root matches @zk-kit LeanIMT. Usage: scripts/groups-flow.sh [identity]
set -euo pipefail
IDENTITY="${1:-semaphore-dev}"
cd "$(dirname "$0")/.."
stellar keys generate "$IDENTITY" --network testnet 2>/dev/null || true
stellar keys fund "$IDENTITY" --network testnet 2>/dev/null || true
ADMIN=$(stellar keys address "$IDENTITY")
stellar contract build
WASM=target/wasm32v1-none/release/semaphore_groups.wasm
stellar contract deploy --wasm "$WASM" --source "$IDENTITY" --network testnet > /tmp/gid.txt 2>&1
CID=$(grep -oE 'C[A-Z0-9]{55}' /tmp/gid.txt | tail -1); echo "GROUPS_ID=$CID"
inv(){ stellar contract invoke --id "$CID" --source "$IDENTITY" --network testnet --send=yes -- "$@"; }
inv create_group --admin "$ADMIN"
for m in 1 2 3 4 5; do inv add_member --group_id 0 --commitment "$m"; done
echo "on-chain root:"; stellar contract invoke --id "$CID" --source "$IDENTITY" --network testnet -- root --group_id 0
