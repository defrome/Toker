const state = {
  user: loadJson("tg_user", null),
  auth: loadJson("tg_auth", null),
  gifts: [],
  currencyFilter: "all",
};

const authControls = document.getElementById("authControls");
const loginPanel = document.getElementById("loginPanel");
const marketPanel = document.getElementById("marketPanel");
const loginForm = document.getElementById("loginForm");
const giftsGrid = document.getElementById("giftsGrid");
const userMeta = document.getElementById("userMeta");
const toast = document.getElementById("toast");

bindEvents();
bootstrap();

function bindEvents() {
  loginForm.addEventListener("submit", onLogin);

  document.querySelectorAll(".filter").forEach((btn) => {
    btn.addEventListener("click", () => {
      state.currencyFilter = btn.dataset.currency;
      document
        .querySelectorAll(".filter")
        .forEach((x) => x.classList.toggle("active", x === btn));
      renderGifts();
    });
  });
}

async function bootstrap() {
  renderAuthControls();

  if (!state.auth?.access_token || !state.user?.tg_id) {
    return;
  }

  const ok = await ensureSession();
  if (ok) {
    await loadGifts();
  } else {
    resetSession();
  }
}

async function onLogin(event) {
  event.preventDefault();

  const tgId = Number(document.getElementById("tgId").value);
  const username = document.getElementById("username").value.trim() || null;
  const wallet = document.getElementById("wallet").value.trim() || null;

  if (!Number.isInteger(tgId) || tgId <= 0) {
    showToast("Telegram ID должен быть положительным числом", true);
    return;
  }

  try {
    const result = await api("/api/auth/login", {
      method: "POST",
      body: {
        tg_id: tgId,
        username,
        wallet_address: wallet,
      },
    });

    state.user = result.user;
    state.auth = result.auth;
    persistSession();

    renderAuthControls();
    loginPanel.classList.add("hidden");
    marketPanel.classList.remove("hidden");

    showToast("Вход успешен. Токены обновлены.");
    await loadGifts();
  } catch (err) {
    showToast(err.message, true);
  }
}

async function ensureSession() {
  try {
    await apiAuth(`/api/users/${state.user.tg_id}`);
    loginPanel.classList.add("hidden");
    marketPanel.classList.remove("hidden");
    return true;
  } catch (_) {
    try {
      await refreshTokens();
      await apiAuth(`/api/users/${state.user.tg_id}`);
      loginPanel.classList.add("hidden");
      marketPanel.classList.remove("hidden");
      return true;
    } catch {
      return false;
    }
  }
}

async function loadGifts() {
  try {
    const gifts = await apiAuth("/api/gifts");
    state.gifts = Array.isArray(gifts) ? gifts : [];
    userMeta.textContent = `@${state.user.username || "anonymous"} · id ${state.user.tg_id}`;
    renderGifts();
  } catch (err) {
    showToast(err.message, true);
  }
}

function renderGifts() {
  giftsGrid.innerHTML = "";

  const visible = state.gifts.filter((gift) => {
    if (state.currencyFilter === "all") return true;
    return gift.currency === state.currencyFilter;
  });

  if (!visible.length) {
    giftsGrid.innerHTML = "<p>Нет подарков для выбранного фильтра.</p>";
    return;
  }

  visible.forEach((gift, idx) => {
    const card = document.createElement("article");
    card.className = "gift-card";
    card.style.animationDelay = `${idx * 0.04}s`;

    const image = gift.image_url
      ? `<img class=\"gift-image\" src=\"${escapeHtml(gift.image_url)}\" alt=\"${escapeHtml(gift.name)}\" />`
      : `<div class=\"gift-image\" style=\"display:grid;place-items:center;color:#6d86a8;\">NO IMAGE</div>`;

    const priceClass = gift.currency === "rub" ? "rub" : "stars";
    const price = formatPrice(gift.price, gift.currency);

    card.innerHTML = `
      ${image}
      <h3>${escapeHtml(gift.name)}</h3>
      <p>${escapeHtml(gift.description)}</p>
      <div class="gift-meta">
        <span class="badge">${escapeHtml(gift.rarity_level)}</span>
        <strong class="price ${priceClass}">${price}</strong>
      </div>
      <button class="buy-btn" ${gift.is_available ? "" : "disabled"}>
        ${gift.is_available ? "Купить" : "Продан"}
      </button>
    `;

    const buyBtn = card.querySelector(".buy-btn");
    if (gift.is_available) {
      buyBtn.addEventListener("click", () => purchaseGift(gift.id, buyBtn));
    }

    giftsGrid.appendChild(card);
  });
}

async function purchaseGift(giftId, btn) {
  btn.disabled = true;
  btn.textContent = "Покупка...";

  try {
    await apiAuth("/api/orders/purchase", {
      method: "POST",
      body: { gift_id: giftId },
    });

    showToast("Покупка создана");
    await loadGifts();
  } catch (err) {
    btn.disabled = false;
    btn.textContent = "Купить";
    showToast(err.message, true);
  }
}

async function apiAuth(path, options = {}) {
  if (!state.auth?.access_token) {
    throw new Error("Сессия отсутствует. Нужен логин.");
  }

  try {
    return await api(path, {
      ...options,
      headers: {
        ...(options.headers || {}),
        Authorization: `Bearer ${state.auth.access_token}`,
      },
    });
  } catch (err) {
    if (err.status !== 401) throw err;
    await refreshTokens();
    return api(path, {
      ...options,
      headers: {
        ...(options.headers || {}),
        Authorization: `Bearer ${state.auth.access_token}`,
      },
    });
  }
}

async function refreshTokens() {
  if (!state.auth?.refresh_token) {
    throw new Error("Refresh token отсутствует");
  }

  const result = await api("/api/auth/refresh", {
    method: "POST",
    body: { refresh_token: state.auth.refresh_token },
  });

  state.auth = result.auth;
  persistSession();
}

async function api(path, options = {}) {
  const response = await fetch(path, {
    method: options.method || "GET",
    headers: {
      "Content-Type": "application/json",
      ...(options.headers || {}),
    },
    body: options.body ? JSON.stringify(options.body) : undefined,
  });

  let payload = null;
  try {
    payload = await response.json();
  } catch {
    payload = null;
  }

  if (!response.ok) {
    const error = new Error(payload?.error || `HTTP ${response.status}`);
    error.status = response.status;
    throw error;
  }

  return payload;
}

function renderAuthControls() {
  authControls.innerHTML = "";

  if (!state.user?.tg_id) {
    return;
  }

  const profile = document.createElement("button");
  profile.className = "ghost";
  profile.textContent = `ID ${state.user.tg_id}`;

  const logout = document.createElement("button");
  logout.className = "primary";
  logout.textContent = "Выйти";
  logout.addEventListener("click", () => {
    resetSession();
    showToast("Сессия очищена");
  });

  authControls.append(profile, logout);
}

function resetSession() {
  state.user = null;
  state.auth = null;
  state.gifts = [];
  localStorage.removeItem("tg_user");
  localStorage.removeItem("tg_auth");

  renderAuthControls();
  giftsGrid.innerHTML = "";
  userMeta.textContent = "";
  marketPanel.classList.add("hidden");
  loginPanel.classList.remove("hidden");
}

function persistSession() {
  localStorage.setItem("tg_user", JSON.stringify(state.user));
  localStorage.setItem("tg_auth", JSON.stringify(state.auth));
}

function loadJson(key, fallback) {
  try {
    return JSON.parse(localStorage.getItem(key)) || fallback;
  } catch {
    return fallback;
  }
}

function formatPrice(price, currency) {
  if (currency === "rub") {
    return `${new Intl.NumberFormat("ru-RU").format(price)} ₽`;
  }
  return `${new Intl.NumberFormat("ru-RU").format(price)} ⭐`;
}

function showToast(message, isError = false) {
  toast.textContent = message;
  toast.style.borderColor = isError ? "rgba(251,113,133,0.6)" : "rgba(45,212,191,0.5)";
  toast.classList.add("show");
  setTimeout(() => toast.classList.remove("show"), 2200);
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}
