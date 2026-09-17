# Shared helpers for the Artemis harness adapter (sourced, not executed).

ARTEMIS_REPO_KEY="roaring-rs"

sync_workspace() {
    local checkout
    checkout="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
    WORKSPACE="${HARD_OSS_WORKSPACE_ROOT:-/var/tmp/hard-oss-workspaces}/${ARTEMIS_REPO_KEY}"
    mkdir -p "$WORKSPACE"
    rsync -a --delete --exclude '/.git/' --exclude '/target/' --exclude 'artemis_results.json*' \
        "$checkout/" "$WORKSPACE/"
    export WORKSPACE
    export CARGO_TARGET_DIR="$WORKSPACE/target"
    export RUSTFLAGS="-C target-cpu=x86-64-v3"
    export PATH="$HOME/.cargo/bin:$PATH"
    echo "artemis: workspace synced to $WORKSPACE" >&2
}
