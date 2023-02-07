# achvgames-com

```sh
source ./env

cargo sqlx database drop -y
cargo sqlx database setup --source db/migrations
cargo run
```

```sh
docker run \
  --rm \
  --interactive \
  --tty \
  --publish 5432:5432 \
  --env POSTGRES_PASSWORD=postgres \
  postgres
```
