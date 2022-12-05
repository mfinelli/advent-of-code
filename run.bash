#!/usr/bin/env bash

set -ex

if [[ $# -ne 0 ]]; then
  echo >&2 "usage: $(basename "$0")"
  exit 1
fi

YEAR="$(date '+%Y')"
DAY="$(date '+%d')"
INPUT="./input/${YEAR}/day${DAY}.txt"

if [[ ! -f $INPUT ]]; then
  echo >&2 "error: can't find input $INPUT"
  exit 1
fi

cargo run -- "y${YEAR:2}d${DAY}" "$INPUT"

exit 0
