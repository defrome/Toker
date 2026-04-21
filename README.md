# TG Gifts Market API

<p align="center">
  <img src="https://img.shields.io/badge/Rust-API-orange?style=for-the-badge&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Axum-0.7-1f6feb?style=for-the-badge" alt="Axum" />
  <img src="https://img.shields.io/badge/LibSQL-Database-0ea5e9?style=for-the-badge" alt="LibSQL" />
  <img src="https://img.shields.io/badge/JWT-Rotation-22c55e?style=for-the-badge" alt="JWT Rotation" />
</p>

<p align="center">
  🎁 API для маркетплейса подарков Telegram с авторизацией, покупками и управлением каталогом
</p>

---

## ✨ Что внутри

- 🔐 JWT auth с `access + refresh` и ротацией refresh-токенов
- 🧠 Безопасное хранение только SHA-256 хэшей refresh-токенов
- 🛍️ Каталог подарков, заказы и статусы заказа
- 💸 Цены в `⭐ Stars` и `₽ RUB` (без TON)
- 🧾 Swagger UI для тестирования API

## 🧩 Стек

- `Rust + Tokio`
- `Axum`
- `LibSQL / SQLite-compatible`
- `Utoipa + Swagger UI`

## 🚀 Быстрый старт

1. Скопируй env:

```bash
cp .env.example .env
```

2. Запусти сервер:

```bash
cargo run
```

3. Открой Swagger UI:

- `http://127.0.0.1:3000/swagger-ui`

## ⚙️ ENV

```env
DATABASE_URL=file:marketplace.db
JWT_ACCESS_SECRET=replace_with_long_random_access_secret
JWT_REFRESH_SECRET=replace_with_long_random_refresh_secret
JWT_ISSUER=my-site
JWT_AUDIENCE=my-site-api
RUST_LOG=my_site=debug,tower_http=info
# LIBSQL_AUTH_TOKEN=replace_with_turso_token
```

Для Turso/remote LibSQL:

```env
DATABASE_URL=libsql://<your-db>.turso.io
LIBSQL_AUTH_TOKEN=<token>
```

## 🔐 Auth виджет

| Тип | TTL | Секрет | Особенность |
|---|---:|---|---|
| `access_token` | 1 час | `JWT_ACCESS_SECRET` | для защищённых endpoint'ов |
| `refresh_token` | 30 дней | `JWT_REFRESH_SECRET` | одноразовый (rotation) |

- Claims: `iss`, `aud`, `iat`, `nbf`, `jti`, `token_type`, `sub`
- После `POST /api/auth/refresh` старый refresh токен становится недействительным

## 🌐 API маршруты

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

## 💎 Валюта подарков

- `currency: "stars"` → Telegram Stars `⭐`
- `currency: "rub"` → Russian Ruble `₽`
- `price` хранится как целое число

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
