## 概要

Rust勉強用リポジトリ

## Mockサーバー起動手順

- Dockerイメージのビルド
  - `docker build -t mock-api .`
- Dockerコンテナの起動
  - `docker run -p 4010:4010 mock-api`
- `http://localhost:4010/customers`にアクセスして、モックデータ（`./mock/db.jso`）が表示されればOK
