# Reproducing memory leak

Run nats

`docker run -p 4222:4222 -p 8222:8222 -p 6222:6222 --rm --name
nats-server -ti nats:latest`

Build and run consumer and producer in different terminal sessions
```
cargo run --release -p nats-sender
cargo run --release -p nats-eater
```
