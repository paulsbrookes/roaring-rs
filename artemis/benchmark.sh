#!/usr/bin/env bash
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CHECKOUT="$(cd "$HERE/.." && pwd)"
source "$HERE/lib.sh"
rm -f "$CHECKOUT/artemis_results.json" "$CHECKOUT/artemis_results.json.tmp"
sync_workspace
rm -f "$WORKSPACE/artemis_results.json" "$WORKSPACE/artemis_results.json.tmp"
# Ensure the workspace build matches the synced source (incremental no-op when unchanged).
bash "$HERE/compile.sh"
cd "$WORKSPACE"
BIN="$WORKSPACE/target/release/artemis-bench"
[ -x "$BIN" ] || { echo "artemis: $BIN missing after compile" >&2; exit 1; }
echo "artemis: running artemis-bench" >&2
"$BIN" "$WORKSPACE/artemis_results.json.tmp"
cp "$WORKSPACE/artemis_results.json.tmp" "$CHECKOUT/artemis_results.json.tmp"
mv "$CHECKOUT/artemis_results.json.tmp" "$CHECKOUT/artemis_results.json"
cat "$CHECKOUT/artemis_results.json" >&2
