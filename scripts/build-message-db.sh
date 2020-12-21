#!/usr/bin/env bash

if [ -z "$1" ]
then
  echo "Please specify the version of message-db to build. For example, 1.2.3 (https://github.com/message-db/message-db)"
  exit 1
fi


scripts_dir=$(dirname "${BASH_SOURCE[0]}")
name="mbriggs/message-db"
tversion="$name:$1"
tlatest="$name:latest"

pushd "$scripts_dir/../message-db" || exit 1

docker build --no-cache --build-arg MESSAGE_DB_VERSION="$1" -t "$tversion" -t "$tlatest" . || exit 1
docker push "$tversion"
docker push "$tlatest"

popd || return