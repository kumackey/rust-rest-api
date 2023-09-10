# 発表アジェンダ

## デモ

みんなも触ってみよう！

```
curl https://nuuuuuuuuuuu-8389623ca042.herokuapp.com/users
curl -X POST http://localhost:8080/users -d '{"name":"hoge"}' -H 'Content-Type: application/json'
curl https://nuuuuuuuuuuu-8389623ca042.herokuapp.com/users
```

## 学んだこと・取り組んだこと

### kumackey

- web API のフレームワークとしては actix-web が有名(本ハンズオンでも採用)
- test は production code に直接書ける

### kerochelo

### kenchasonakai

### miloneko

値は常に所有者を持つ

```
fn main() {
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫

} // ここでxがスコープを抜け、sもスコープを抜ける。だが、sの値はムーブされているので、何も特別なことはない。
  //
```

所有権は一意で移動可能

```
fn main() {
   let s1 = String::from("hello"); // s1がスコープに入る

   let s2 = s1;                    // s1の値がs2にムーブする
                                   // これ以降、s1は無効になる

   println!("{}, world!", s1);     // エラーになる
}
```

所有権がスコープを抜けると、自動的に解放
fn main() {
let s = String::from("hello"); // s がスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫

} // ここで x がスコープを抜け、s もスコープを抜ける。だが、s の値はムーブされているので、何も特別なことは起きない。

所有権によるメモリ管理のメリット

- プログラムにより明示的にメモリ領域の確保と解放を行う
- ガベージコレクションにより不要なメモリ領域を自動的に解放する

```
fn main() {
    let s1 = String::from("hello"); // s1がスコープに入る

    let (s2, len) = calculate_length(s1); // s1の所有権がmoveされ、s2にmoveされる

    println!("The length of '{}' is {}.", s2, len); // s2がスコープに入る
} // ここでs2はスコープを抜け、dropされる。s1もスコープを抜けるが、所有権がムーブされているので何も起きない

fn calculate_length(s: String) -> (String, usize) { // sがスコープに入る
    let length = s.len(); // lengthがスコープに入る

    (s, length) // sとlengthを返す
} // ここでlengthがスコープを抜ける。sもスコープを抜けるが、所有権がムーブされているので何も起きない
```

所有権は複数の参照を持つことができないが、GC は複数の参照を持つことができる

```
rustのコード
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 6;
    println!("{}", x);
}

JavaScriptのコード
var x = 5;
var y = x; y = 6;
console.log(x);
// 5
```

### watsumi

- actix-web
- deisel

# 開発

```bash
# ユーザー一覧取得
curl http://localhost:8080/users

# ユーザからの回答提出
curl -X POST http://localhost:8080/qustions/1/answers -d '{"answer": "阿蘇山", "user_name":"kumackey"}' -H 'Content-Type: application/json'
```

## データベース

### 設計

![ER図](./er.png "ER図")

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

## テスト

```bash
# この一行はアプリケーションコードを変更したときのみ。もしかしたらbind mountしてるから要らないかも？
docker compose build

docker compose up -d
docker compose exec rust cargo test
```

ちなみにデータベースは分けてないので、テストのたびに色々データが作り直されます。

## web api サーバ立ち上げ

### docker で起動する場合(推奨)

```
docker compose up -d
```

http://localhost:8080/

アプリケーションコードをいじった場合は docker compose up -d --build で再起動します

### local の host machine で起動(非推奨)

```bash
cargo run
```
