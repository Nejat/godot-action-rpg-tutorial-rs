#!/bin/bash

if [ -z "$1" ]; then
  build="release"
else
  build=$1
fi

echo

if [[ "$build" == "release" ]]; then
  echo building release ...
  echo
  cargo build --release || { exit 1; }
elif [[ "$build" == "debug" ]]; then
  echo building debug ...
  echo
  cargo build || { exit 1; }
else
  echo "$1 is an invalid argument"
  echo
  echo "usage: ./build.sh [debug|release]"
  echo
  exit 1
fi

echo
echo "copying $build libaction_rpg_rs.so to ../action-rpg/libaction_rpg_rs.so"
echo

cp "./target/$build/libaction_rpg_rs.so" ../action-rpg/libaction_rpg_rs.so || { exit 1; }

echo "done"
echo
