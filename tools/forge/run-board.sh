#!/usr/bin/env bash
set -euo pipefail
if [[ $# != 3 ]]; then
    echo "usage: run-board.sh MANIFEST TRACE MAX_BOUNDARIES" >&2
    exit 1
fi
manifest_path=$(realpath "$1")
trace_path=$(realpath -m "$2")
limit=$3
source "$(dirname "$0")/engine.sh"
project_path=$PWD
classpath_path="$project_path/target/forge/classpath.txt"
mkdir -p target/forge/classes
if [[ ! -f "$classpath_path" ]]; then
    classpath_build=$(mktemp "$project_path/target/forge/classpath.XXXXXX")
    trap 'rm -f -- "$classpath_build"' EXIT
    (
        cd "$engine"
        mvn -B -ntp -pl forge-gui-desktop -am test-compile dependency:build-classpath \
            -DincludeScope=test -Dmdep.outputFile="$classpath_build" -Dcheckstyle.skip
    )
    mv "$classpath_build" "$classpath_path"
    trap - EXIT
fi
engine_classpath="$project_path/$engine/forge-gui-desktop/target/classes:$(cat "$classpath_path")"
javac -cp "$engine_classpath" -d target/forge/classes \
    tools/forge/MtgBoard.java tools/forge/MtgController.java tools/forge/MtgTrace.java tools/forge/MtgRunner.java
cd "$engine/forge-gui-desktop"
exec java -Xmx4g -cp "$project_path/target/forge/classes:$engine_classpath" \
    forge.ai.MtgRunner "$manifest_path" "$trace_path" "$limit"
