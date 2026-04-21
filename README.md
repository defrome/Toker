# Telegram NFT Gift Marketplace API (Rust + LibSQL)

Бэкенд REST API для маркетплейса Telegram NFT Gifts.

Технологии:
- Rust + Tokio (асинхронно)
- Axum
- LibSQL (SQLite-compatible)
- Utoipa + Swagger UI

## Запуск

1. Проверь `.env`:

```env
DATABASE_URL=file:marketplace.db
RUST_LOG=my_site=debug,tower_http=info
```

Для remote LibSQL (Turso):

```env
DATABASE_URL=libsql://<your-db>.turso.io
LIBSQL_AUTH_TOKEN=<token>
```

2. Запусти:

```bash
cargo run
```

3. Swagger UI:
- `http://127.0.0.1:3000/swagger-ui`

## Миграция

SQL схема лежит в `schema.sql` и автоматически применяется при старте.

## Основные endpoints

- `GET /api/health`
- `POST /api/gifts`
- `GET /api/gifts`
- `GET /api/gifts/{id}`
- `PUT /api/gifts/{id}`
- `DELETE /api/gifts/{id}`
- `POST /api/users`
- `GET /api/users/{tg_id}`
- `POST /api/orders/purchase`
- `GET /api/orders/{id}`
- `PATCH /api/orders/{id}/status`
