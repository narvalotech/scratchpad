#!/usr/bin/env sh

program=$1
kdotool search --name "$program" --limit 1 windowactivate
