# evt

Event-Sourced autonomous services for rust. An implementation of (Eventide)[https://eventide-project.org/] using (Message DB)[https://github.com/message-db/message-db].

**WARNING**: this project is in active development and not yet functional.

### Developing

It is epected that you have `MESSAGE_STORE_URL`. If you do not want to set this globally, https://direnv.net/ can be useful for setting things like this per project.

A convenience script is included for quickly setting up and running message-db with docker. `./scripts/provision-db.sh`

If you use the docker image, this is the connection string: `export MESSAGE_STORE_URL="postgres://message_store:message_store@localhost:5432"`
