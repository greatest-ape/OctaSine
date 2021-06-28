#/bin/bash

set -e

if [[ -z $1 ]]; then
    echo "Usage: $0 [bundle path]"
    echo "Example: $0 ./tmp/OctaSine.vst"
else
    MOVE_FROM="$1"

    NAME="OctaSine"
    VST_NAME="$NAME.vst"
    MOVE_TO="/Library/Audio/Plug-Ins/VST/$VST_NAME"

    if [ -d "$MOVE_TO" ]; then
        rm -r "$MOVE_TO"
    fi

    cp -r "$MOVE_FROM" "$MOVE_TO"

    echo "Copied VST bundle from $MOVE_FROM to $MOVE_TO"
fi
