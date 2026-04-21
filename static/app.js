const giftGrid = document.getElementById("giftGrid");
const giftCount = document.getElementById("giftCount");
const orderOutput = document.getElementById("orderOutput");
const giftCardTemplate = document.getElementById("giftCardTemplate");

const tgIdInput = document.getElementById("tgIdInput");
const usernameInput = document.getElementById("usernameInput");
const walletInput = document.getElementById("walletInput");

const refreshBtn = document.getElementById("refreshBtn");
const saveUserBtn = document.getElementById("saveUserBtn");

let currentUserId = null;

const fallbackImage =
  "data:image/svg+xml;utf8," +
  encodeURIComponent(
    '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 500 500"><defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1"><stop stop-color="#0f3a66"/><stop offset="1" stop-color="#081324"/></linearGradient></defs><rect width="500" height="500" fill="url(#g)"/><circle cx="250" cy="210" r="88" fill="#00a6ff" fill-opacity="0.35"/><text x="50%" y="82%" dominant-baseline="middle" text-anchor="middle" fill="#b6e5ff" font-family="Arial" font-size="38">TON NFT GIFT</text></svg>'
  );

function setStatus(data) {
  orderOutput.textContent = typeof data === "string" ? data : JSON.stringify(data, null, 2);
}

async function api(path, options = {}) {
  const response = await fetch(path, {
    headers: { "Content-Type": "application/json" },
    ...options,
  });

  const raw = await response.text();
  let body;

  try {
    body = raw ? JSON.parse(raw) : null;
  } catch {
    body = raw;
  }

  if (!response.ok) {
    throw new Error(body?.error || body || `HTTP ${response.status}`);
  }

  return body;
}

function formatTonFromCents(cents) {
  return `${(Number(cents) / 100).toFixed(2)} TON`;
}

function buildGiftCard(gift) {
  const node = giftCardTemplate.content.cloneNode(true);

  const image = node.querySelector(".gift-image");
  image.src = gift.image_url || fallbackImage;
  image.onerror = () => {
    image.src = fallbackImage;
  };

  node.querySelector(".gift-rarity").textContent = gift.rarity_level;
  node.querySelector(".gift-slug").textContent = `#${gift.slug}`;
  node.querySelector(".gift-name").textContent = gift.name;
  node.querySelector(".gift-description").textContent = gift.description;
  node.querySelector(".gift-price").textContent = formatTonFromCents(gift.price);

  const buyBtn = node.querySelector(".buy-btn");
  if (!gift.is_available) {
    buyBtn.disabled = true;
    buyBtn.textContent = "Продано";
  } else {
    buyBtn.addEventListener("click", () => purchaseGift(gift.id, buyBtn));
  }

  return node;
}

async function loadGifts() {
  giftGrid.innerHTML = "";

  try {
    const gifts = await api("/api/gifts");
    giftCount.textContent = `${gifts.length} лотов`;

    if (!gifts.length) {
      setStatus("Каталог пуст. Добавь подарки через API /api/gifts.");
      return;
    }

    gifts.forEach((gift) => giftGrid.appendChild(buildGiftCard(gift)));
  } catch (error) {
    setStatus(`Ошибка загрузки каталога: ${error.message}`);
  }
}

async function saveUser() {
  const tgId = Number(tgIdInput.value);
  if (!Number.isInteger(tgId) || tgId <= 0) {
    setStatus("Telegram ID должен быть положительным целым.");
    return;
  }

  try {
    const user = await api("/api/users", {
      method: "POST",
      body: JSON.stringify({
        tg_id: tgId,
        username: usernameInput.value.trim() || null,
        wallet_address: walletInput.value.trim() || null,
      }),
    });

    currentUserId = user.tg_id;
    setStatus({ message: "Пользователь сохранен", user });
  } catch (error) {
    setStatus(`Ошибка сохранения пользователя: ${error.message}`);
  }
}

async function purchaseGift(giftId, button) {
  const tgId = Number(tgIdInput.value);

  if (!Number.isInteger(tgId) || tgId <= 0) {
    setStatus("Перед покупкой укажи Telegram ID.");
    return;
  }

  if (currentUserId !== tgId) {
    setStatus("Сначала нажми 'Сохранить'.");
    return;
  }

  button.disabled = true;
  button.textContent = "Покупка...";

  try {
    const result = await api("/api/orders/purchase", {
      method: "POST",
      body: JSON.stringify({ user_id: tgId, gift_id: giftId }),
    });

    setStatus({ message: "Покупка успешна", result });
    await loadGifts();
  } catch (error) {
    setStatus(`Ошибка покупки: ${error.message}`);
    button.disabled = false;
    button.textContent = "Купить";
  }
}

refreshBtn.addEventListener("click", loadGifts);
saveUserBtn.addEventListener("click", saveUser);

loadGifts();
