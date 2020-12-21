#!/bin/sh

if [ -z "$MESSAGEDB_PASSWORD" ]
then
  echo "Please specify the password to be used for the message-db user by setting \$MESSAGEDB_PASSWORD"
  exit 1
fi

cd /usr/src/message-db/database || exit 1
./install.sh
psql -c "ALTER ROLE message_store WITH PASSWORD '$MESSAGEDB_PASSWORD'"