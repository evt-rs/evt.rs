#!/usr/bin/env bash

if [ -z "$MESSAGE_DB_PATH" ]
then
  echo "Please export \$MESSAGE_DB_PATH as a directory that message-db code exists (https://github.com/message-db/message-db)"
  exit 1
fi

echo "-- removing evt-pg container --"
docker stop evt-pg
docker rm evt-pg
docker volume rm evt-pg
echo "-- done --"

echo "-- provisioning --"
docker volume create evt-pg

docker create \
  --name evt-pg \
  -e POSTGRES_PASSWORD=evt \
  -p 5432:5432 \
  --mount "src=evt-pg,dst=/var/lib/postgresql/data" \
  postgres:12

docker start evt-pg

until docker run --rm \
  --link evt-pg:pg \
  postgres:12 pg_isready \
    -U postgres \
    -h pg
do
  sleep 1
done

docker run --rm \
  --link evt-pg:pg \
  -e PGUSER=postgres \
  -e PGPASSWORD=evt \
  -e PGHOST=pg \
  --mount "type=bind,src=$MESSAGE_DB_PATH,dst=/message-db" \
  -w /message-db/database \
  postgres:12 bash ./install.sh

docker run --rm \
  --link evt-pg:pg \
  -e PGUSER=postgres \
  -e PGPASSWORD=evt \
  -e PGHOST=pg \
  postgres:12 psql -c "ALTER ROLE message_store WITH PASSWORD 'message_store'"