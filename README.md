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
```

### マイグレーション

```bash
docker compose exec rust diesel migration run
```

マイグレーションが成功したかは、上記のデータベース接続で select するなりで確認してください。

## web api サーバ立ち上げ

### docker で起動する場合(推奨)
```
cp .env.docker.default .env
docker compose up -d
```

http://localhost:8080/

### local(非推奨)
```bash
cargo run
```


