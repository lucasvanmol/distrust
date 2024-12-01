# Distrust: **Dist**ributed Systems in **Rust**

Implementing [fly.io](https://fly.io/dist-sys/) distributed systems challenges in Rust. 

Includes:
- [x] echo
- [x] unique-id generator
- [ ] gossip protocol
- [ ] grow-only counter
- [ ] kafka-style log

## Running

First start up maelstrom. This will:
- compile the existing rust binaries and copy them into the container
- start up the maelstrom ui at localhost:8080
- enable the container to run the binaries with the commands that follow

```
docker compose up --build -d
```

## Testing

### echo

```
docker exec -it maelstrom maelstrom test -w echo --bin ./bin/echo --node-count 1 --time-limit 10
```

### unique-id generator:

```
docker exec -it maelstrom maelstrom test -w unique-ids --bin ./bin/uid --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```

### gossip

```
docker exec -it maelstrom maelstrom test -w broadcast --bin ./bin/broadcast --node-count 1 --time-limit 20 --rate 10
```