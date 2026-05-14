#!/usr/bin/env sh

program=$@

wmctrl -i -a $(wmctrl -l | grep -i "$program" | tail -n 1 | cut -d ' ' -f1)
