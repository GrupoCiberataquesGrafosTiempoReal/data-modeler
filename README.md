# data-modeler

This program reads messages from a specific topic from an Apache Kafka broker and models the obtained data within a CQL query to write it to a Neo4j database.

## Environmental variables

This are the environmental variables used by the program:

```sh
KAFKA_BROKERS=<ip:port>         # Ex.: localhost:9092
KAFKA_TOPIC=<name>              # Ex.: topic.name
BATCH_SIZE=<numer>              # Ex.: 10 (Number of messages per batch)
BATCH_TIMEOUT=<number>          # Ex.: 1000 (Number of miliseconds to wait to process a incomplet batches)
DB_URL=<protocol://ip:port>     # Ex.: neo4j://localhost:7687  
DB_NAME<name>                   # Ex.: neo4j
DB_USER=<user>                  # Ex.: user
DB_PASS=<password>              # Ex.: pass
```

## Run with Cargo

The program is compiled and executed using `cargo`. You can set up all needed tools following these instructions: https://rust-lang.org/tools/install/.

> It is necessary to install `cmake` to compile the project locally.

Compile the program:

```sh
cargo compile
```

Create an environment variables file named `.env` with the required variables.

Run the program:
```sh
cargo run
```

## Run with Docker

Contaner is build and executed with Docker. You can follow these instructions to set up Docker: https://docs.docker.com/engine/install/.

Build the image:

```sh
docker build --no-cache -t data-modeler .
```

Create an environment variables file named `.env` with the required variables.

Run the container:

```sh
docker run \
    --network host \
    --env-file .env \
    --rm data-modeler
```