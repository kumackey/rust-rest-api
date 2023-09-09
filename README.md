# rust-rest-api

## 環境変数設定

```bash
cp .env.default .env
```

## データベース

### データベース接続

```bash
docker compose up -d
docker compose exec postgres bash

psql -U postgres

# 以下は初期接続のときのみ
CREATE DATABASE rust_rest_api;
\c rust_rest_api
```

### データベースリセット

```
docker compose down -v
docker compose up -d
# この場合、上記のCREATE DATABASE rust_rest_api;をやり直して下さい。
```

### diesel_cli の導入

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
```

参考: https://zenn.dev/helloyuki/scraps/a242bfc79576c3

### マイグレーション

```bash
diesel migration run
```

マイグレーションが成功したかは、上記のデータベース接続で select するなりで確認してください。

## web api サーバ立ち上げ

```bash
cargo run
```

### docker で起動する場合

```
cp .env.docker.default .env
docker compose up -d
docker compose exec rust bash
diesel migration run
cargo run
```

http://localhost:8080/

上記通りやれば
found person: 1 John
って返ってくる(はず)
