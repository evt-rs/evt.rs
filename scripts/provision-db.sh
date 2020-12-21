#!/usr/bin/env bash

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
  -e MESSAGEDB_PASSWORD=message_store \
  -p 5432:5432 \
  --mount "src=evt-pg,dst=/var/lib/postgresql/data" \
  mbriggs/message-db:latest

docker start evt-pg

until docker run --rm \
  --link evt-pg:pg \
  postgres:12 pg_isready \
    -U postgres \
    -h pg
do
  sleep 1
done

docker logs evt-pg