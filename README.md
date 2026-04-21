# Telegram Gifts Marketplace API (Rust + LibSQL)

Бэкенд REST API и веб-клиент для маркетплейса Telegram Gifts.

Технологии:
- Rust + Tokio
- Axum
- LibSQL (SQLite-compatible)
- Utoipa + Swagger UI
- Vanilla JS frontend (`/app`)

## Запуск

1. Проверь `.env`:

```env
DATABASE_URL=file:marketplace.db
JWT_ACCESS_SECRET=change_me_to_long_random_access_secret
JWT_REFRESH_SECRET=change_me_to_long_random_refresh_secret
JWT_ISSUER=my-site
JWT_AUDIENCE=my-site-api
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

3. Интерфейсы:
- Frontend: `http://127.0.0.1:3000/app`
- Swagger UI: `http://127.0.0.1:3000/swagger-ui`

## JWT аутентификация (реализовано)

- `access_token` (1 час) и `refresh_token` (30 дней) подписываются разными секретами.
- В claims добавлены `iss`, `aud`, `iat`, `nbf`, `jti`, `token_type`.
- Refresh-токены одноразовые (rotation): после успешного `/api/auth/refresh` старый refresh помечается использованным.
- В БД хранится только SHA-256 хэш refresh-токена (`auth_refresh_tokens`), а не сам токен.

## Основные endpoints

- `GET /api/health`
- `POST /api/auth/login`
- `POST /api/auth/refresh`
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

## Валюта подарков

Вместо TON у подарков теперь:
- `currency: "stars"` (Telegram Stars)
- `currency: "rub"` (рубли)

Поля подарка:
- `price` (целое число)
- `currency` (`stars` | `rub`)

Пример создания подарка:

```json
POST /api/gifts
{
  "slug": "golden-fox",
  "name": "Golden Fox",
  "description": "Limited TG gift",
  "image_url": "https://...",
  "price": 1200,
  "currency": "stars",
  "rarity_level": "legendary",
  "is_available": true
}
```

## Фронтенд

`/app` это TG Gifts-style UI:
- логин через `POST /api/auth/login`
- авто-refresh access токена через `POST /api/auth/refresh`
- каталог подарков с фильтрами по валюте (`⭐` / `₽`)
- покупка через `POST /api/orders/purchase`
