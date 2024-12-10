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

library="libaction_rpg_rs"

case "$(uname -sr)" in
   Darwin*)
     library="$library.dynlib"
     ;;

   Linux*|Linux*Microsoft*|CYGWIN*|MINGW*|MINGW32*|MSYS*)
     library="$library.so"
     ;;

   *)
     library="$library.so"
     ;;
esac

echo
echo "copying $build $library to ../action-rpg/$library"
echo

cp "./target/$build/$library" ../action-rpg/$library || { exit 1; }

echo "done"
echo
