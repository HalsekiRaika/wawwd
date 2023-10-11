# WAWWD API Server

Required 
* Docker
* Docker-Compose
* rustup 1.72+

Set Environment Variable
```shell
touch .env.docker
echo "PG_DATABASE_URL=postgresql://wawwd123:wawwd456@postgres_db:5432/wawwd" >> .env.docker
```

Run On
```shell
docker-compose up -d
```

Run Dev
```shell
docker-compose -f docker-compose-test.yml up -d
```

```shell
cargo run -- --debug-assertion
```
