#!/usr/bin/env bash
# Shared pinned checkout and helper sources for the runner and engine tests.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../.."
revision=5222f05e569da8e33c85858e6fe1698901548d64
engine=references/forge
if [[ ! -d "$engine/.git" ]]; then
    git clone --filter=blob:none --no-checkout https://github.com/Card-Forge/forge.git "$engine"
    git -C "$engine" checkout "$revision"
fi
if [[ "$(git -C "$engine" rev-parse HEAD)" != "$revision" ]]; then
    echo "Forge checkout must be at $revision" >&2
    exit 1
fi
for helper in tools/forge/*.java; do
    cp "$helper" "$engine/forge-gui-desktop/src/test/java/forge/ai/"
done
