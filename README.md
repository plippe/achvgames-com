# achvgames-com

```sh
source ./env

cargo sqlx database drop -y
cargo sqlx database setup --source db/migrations
cargo run
```
