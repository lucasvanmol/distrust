# Distributed Systems in Rust

How to build and run:

```
docker compose up --build
```

```
docker exec -it maelstrom ./maelstrom/maelstrom test -w echo --bin ./bin/echo --node-count 1 --time-limit 10
```

```
docker exec -it maelstrom ./maelstrom/maelstrom test -w unique-ids --bin ./bin/uid --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```