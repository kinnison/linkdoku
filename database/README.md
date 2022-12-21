# Linkdoku database

To do development here you'll probably want the `diesel_cli` crate installing

> `cargo install diesel_cli --no-default-features --features postgres`

Note, there is a `.env` file present in this directory explicitly to support
the `diesel` CLI tool, it is set up to talk directly with the database
exposed via the docker-compose from the top level of this repo.
