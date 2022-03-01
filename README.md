# ai-meter

## Endpoints
TODO:

## Setup
### Native
1. Install the following:
- [rust](https://www.rust-lang.org/learn/get-started)
- [diesel cli](https://diesel.rs/guides/getting-started)
- [docker](https://docs.docker.com/get-docker/)

2. Setup the db
```sh
cp example.env .env
docker run --name postgres -e POSTGRES_PASSWORD=toor123 -p 5432:5432 -d postgres
# Wait for a few seconds for the db to start
diesel setup
```
- To start the docker db again
```sh
docker start postgress
```

3. Run the server
- Debug: `cargo run`
- Release: `cargo run --release`
- arguments: `cargo run -- --help`

### Docker
TODO:

## Notes
- `insomnia-req-collection.json` is created with and can be imported by [insomnia](https://insomnia.rest/)
- `tables.drawio` is a [diagrams.net](https://www.diagrams.net/) file
