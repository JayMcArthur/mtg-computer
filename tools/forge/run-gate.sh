#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/engine.sh"
fixture_path="$PWD/data/construction/add-halt-prepared.json"
trace_path="$PWD/target/forge/add-halt.jsonl"
constants_path="$PWD/data/construction/constants-prepared.json"
cd "$engine"
mvn -B -ntp -pl forge-gui-desktop -am test \
    -Dtest=MtgGateTest,MtgPreparedBoardTest,MtgConstantBoardTest -Dmtg.fixture="$fixture_path" -Dmtg.constants="$constants_path" -Dmtg.trace="$trace_path" \
    -Dsurefire.failIfNoSpecifiedTests=false -Dcheckstyle.skip
