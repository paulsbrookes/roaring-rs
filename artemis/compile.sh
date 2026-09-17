#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
sync_workspace
cd "$WORKSPACE"
echo "artemis: cargo build --release --locked -p roaring -p artemis-bench" >&2
cargo build --release --locked -p roaring -p artemis-bench
