# WAWWD API Server

Required 
* Docker
* Docker-Compose
* rustup 1.72+

Set Environment Variable
```shell
touch .env.docker
echo "PG_DATABASE_URL=postgresql://wawwd123:wawwd456@postgres_db:5432/wawwd" >> .env.docker
echo "S3_BUCKET_NAME=<bucket_name>" >> .env.docker
echo "S3_BUCKET_REGION=<bucket_region>" >> .env.docker
echo "S3_ANONYMOUS=<boolean: default false>" >> .env.docker
echo "S3_USE_LOCALSTACK=<boolean: default false>" >> .env.docker
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
