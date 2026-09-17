#!/usr/bin/env bash
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$HERE/lib.sh"
sync_workspace
# Ensure the workspace build matches the synced source (incremental no-op when unchanged).
bash "$HERE/compile.sh"
cd "$WORKSPACE"
echo "artemis: running focused correctness gate" >&2
cargo test --release --locked -p roaring --lib \
    --test ops --test union_with --test intersect_with --test difference_with \
    --test symmetric_difference_with --test is_subset --test is_disjoint \
    --test lib --test push --test range_checks --test iter
