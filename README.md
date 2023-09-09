# rust-rest-api

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
docker compose up -d
```

http://localhost:8080/

アプリケーションコードをいじった場合はdocker compose up -d --buildで再起動します

### localのhost machineで起動(非推奨)

```bash
cargo run
```

## デプロイ方法

## 環境変数設定

```bash
cp .env.default .env
```

TODO: 書いておくと楽


