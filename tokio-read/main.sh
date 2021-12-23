#!/bin/sh

set -e

cargo build

for index in $(seq 100000); do
  result=$(target/debug/tokio-read)

  echo $index: "'$result'"

  if [ "$result" != '# toki' ]; then
    exit 1
  fi
done
