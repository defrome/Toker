# TG Gifts Market API

Простое API для маркетплейса подарков Telegram.

## Что умеет

- Авторизация через JWT (`access` + `refresh`)
- Каталог подарков (CRUD)
- Покупка и управление заказами
- Swagger UI для тестов

## Стек

- Rust
- Axum
- LibSQL / SQLite

## Быстрый старт

```bash
cp .env.example .env
cargo run
```

Swagger UI: `http://127.0.0.1:3000/swagger-ui`

## ENV

```env
DATABASE_URL=file:marketplace.db
JWT_ACCESS_SECRET=replace_with_long_random_access_secret
JWT_REFRESH_SECRET=replace_with_long_random_refresh_secret
JWT_ISSUER=my-site
JWT_AUDIENCE=my-site-api
RUST_LOG=my_site=debug,tower_http=info
# LIBSQL_AUTH_TOKEN=replace_with_turso_token
```

Для Turso:

```env
DATABASE_URL=libsql://<your-db>.turso.io
LIBSQL_AUTH_TOKEN=<token>
```

## Основные маршруты

- `GET /api/health`
- `POST /api/auth/login`
- `POST /api/auth/refresh`
- `GET /api/gifts`
- `POST /api/gifts`
- `POST /api/orders/purchase`
