<img src="https://raw.githubusercontent.com/evt-rs/evt.rs/master/logo.svg" alt="Logo Image" height="100" />

<hr/>

Event-Sourced autonomous services for rust. An implementation of [Eventide](https://eventide-project.org/) using [Message DB](https://github.com/message-db/message-db).

**WARNING**: this project is in active development and not yet functional.

### Developing

#### MessageDB

It is expected that you have a `MESSAGE_STORE_URL` environment variable. If you do not want to set this globally, https://direnv.net/ can be useful for setting things like this per project.

A convenience script is included for quickly setting up and running message-db with docker. `./scripts/provision-db.sh`. Re-running this script will remove the previous container, so it can be used to "reset" back to an empty state. The name of the container is `evt-pg`. So after running once, you can stop it with `docker stop evt-pg` and start it again with `docker start evt-pg`.

If you use the docker image, this is the connection string: `export MESSAGE_STORE_URL="postgres://message_store:message_store@localhost:5432"`

#### Logging

[`envlogger`](https://docs.rs/env_logger/0.8.2/env_logger/) is used in development, which somewhat matches the features of the eventide ruby logger. See documentation on how to control output via `RUST_LOG`.
