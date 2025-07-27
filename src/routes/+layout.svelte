<script lang="ts">
  import { page } from "$app/state";
  import { beforeNavigate } from "$app/navigation";
  import { onMount, onDestroy } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { load } from "@tauri-apps/plugin-store";
  import Welcome from './welcome/+page.svelte';
  import { welcomeComplete, language, licenseKey, theme } from '../stores';
  import { t, locale } from '../language';
  import io from "socket.io-client";

  let { children } = $props();

  let isLoading = $state(true);
  let loadingTimeout: NodeJS.Timeout | null = $state(null);
  let isDragging = $state(false);
  let isMouseOver = $state(false);
  let isExpanded = $derived(isDragging || isMouseOver);
  let isSidebarCollapsed = $state(false);
  let showSettings = $state(false);
  let currentTheme = $state('dark');
  let autoLockEnabled = $state(false);
  let autoLockTime = $state(5);
  let isPinging = $state(false);
  let pingSuccess = $state(false);
  let pingProgress = $state(0);
  let activationAttempted = $state(false);

  let socket: ReturnType<typeof io> | null = null;
  let heartbeat: ReturnType<typeof setInterval>;

  let servers = [
    { name: "Main license server", url: "https://db.totlaunloc.top", status: "pending" },
    { name: "Secondary license server", url: "https://api.totlaunloc.top", status: "pending" },
  ];

  onMount(async () => {
    socket = io("https://api.totlaunloc.top", {
      transports: ["websocket"],
      withCredentials: true,
    });

    socket.on("connect", () => {
      console.log("✅ Socket.IO connected");
      console.log("Socket ID:", socket?.id ?? "unavailable");
    });


    socket.on("connect_error", (err) => {
      console.error("❌ Connection error:", err);
    });

    socket.on("disconnect", (reason) => {
      console.warn("⚠️ Socket disconnected:", reason);
    });

    socket.on("reconnect", (attemptNumber) => {
      console.log(`🔁 Reconnected after ${attemptNumber} attempts`);
    });

    socket.on("reconnect_attempt", (attemptNumber) => {
      console.log(`🔄 Reconnect attempt #${attemptNumber}`);
    });

    socket.on("reconnecting", (attemptNumber) => {
      console.log(`⏳ Reconnecting... Attempt #${attemptNumber}`);
    });

    socket.on("reconnect_error", (error) => {
      console.error("❌ Reconnect error:", error);
    });

    socket.on("reconnect_failed", () => {
      console.error("❌ Reconnect failed");
    });

    socket.onAny((event, ...args) => {
      console.log(`📩 Incoming event: "${event}"`, args);
    });

    heartbeat = setInterval(async () => {
      if (socket?.connected) {
        const store = await load("settings.json");
        const machineId = await store.get("randomId");
        const apiKey = await store.get("licenseKey");
        console.log("💓 Sending heartbeat with machineId: " + machineId + " and apiKey: " + apiKey);
        socket.emit("heartbeat", { machineId, apiKey });
      }
    }, 5000);

    const store = await load("settings.json");
    const savedLanguage = await store.get("language");
    const savedTheme = await store.get("theme");
    const savedLicenseKey = await store.get("licenseKey");
    const savedWelcomeComplete = await store.get("welcomeComplete");

    if (savedLanguage) language.set(savedLanguage as string);
    $: locale.set($language);

    if (savedLicenseKey) licenseKey.set(savedLicenseKey as string);
    if (savedWelcomeComplete) welcomeComplete.set(savedWelcomeComplete as boolean);

    isLoading = false;
  });

  onDestroy(() => {
    if (heartbeat) clearInterval(heartbeat);
    if (socket) socket.disconnect();
    console.log("🛑 Socket.IO disconnected and cleaned up");
  });

  async function toggleTheme(theme: 'dark' | 'light') {
    currentTheme = theme;
    if (theme === 'light') {
      window.document.body.classList.add("light-mode-filter");
    } else {
      window.document.body.classList.remove("light-mode-filter");
    }
    const store = await load("settings.json");
    await store.set("theme", theme);
    await store.save();
  }

  const getRandomDelay = () => Math.random() * (1200 - 200) + 200;

  function toggleSidebar() {
    isSidebarCollapsed = !isSidebarCollapsed;
  }

  beforeNavigate(() => {
    if (loadingTimeout) clearTimeout(loadingTimeout);
    isLoading = true;
    const delay = getRandomDelay();
    loadingTimeout = setTimeout(() => {
      isLoading = false;
      loadingTimeout = null;
    }, delay);
  });

  const appWindow = getCurrentWindow();

  function minimize() {
    appWindow.minimize();
  }

  function close() {
    appWindow.close();
  }

  function startDrag() {
    appWindow.startDragging();
  }

  async function setLanguage(langCode: string) {
    const store = await load("settings.json");
    language.set(langCode);
    await store.set("language", langCode);
    await store.save();
    window.location.reload();
  }

  function handleMouseDown(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (target.closest('button')) return;
    isDragging = true;
    startDrag();
    const handleMouseUp = () => {
      isDragging = false;
      window.removeEventListener('mouseup', handleMouseUp);
    };
    window.addEventListener('mouseup', handleMouseUp);
  }

  async function activateLicense() {
    activationAttempted = true;
    isPinging = true;
    pingSuccess = false;
    pingProgress = 0;
    const store = await load("settings.json");
    await store.set("licenseKey", $licenseKey);
    await store.save();

    const totalServers = servers.length;
    let successfulPings = 0;

    for (let i = 0; i < totalServers; i++) {
      try {
        await fetch(servers[i].url, { method: 'HEAD', mode: 'no-cors' });
        servers[i].status = "success";
        successfulPings++;
      } catch (error) {
        servers[i].status = "error";
      } finally {
        pingProgress = ((i + 1) / totalServers) * 100;
        servers = [...servers];
        if (i < totalServers - 1) {
          await new Promise(resolve => setTimeout(resolve, 300));
        }
      }
    }

    if (successfulPings === totalServers) {
      pingSuccess = true;
    }

    isPinging = false;
  }
</script>


{#if $welcomeComplete}
  <div
    class="titlebar"
    class:expanded={isExpanded}
    onmouseenter={() => isMouseOver = true}
    onmouseleave={() => isMouseOver = false}
    role="button"
    tabindex="0"
    onmousedown={handleMouseDown}
  >
    <button class="titlebar-button" id="titlebar-minimize" onclick={minimize}>
      <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
    </button>
    <button class="titlebar-button" id="titlebar-close" onclick={close}>
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </button>
  </div>

  <div class="app-container">
    <aside class="sidebar" class:collapsed={isSidebarCollapsed}>
      <nav class="nav-list">
        <a class="nav-item" class:no-click={!isSidebarCollapsed} href="/"><img src="/icons/Logoo.png" alt="Welcome icon" class="welcome-icon" /></a>
        <a class="nav-item { page.url.pathname === '/passwords' ? 'active' : '' }" href="/passwords">
          <img class="icon" src="/icons/key.svg" alt="Passwords icon" />
          <span>{$t('passwords')}</span>
        </a>
        <a class="nav-item { page.url.pathname === '/one-time-codes' ? 'active' : '' }" href="/one-time-codes">
          <img class="icon" src="/icons/clock.svg" alt="One-Time Codes icon" />
          <span>{$t('one_time_codes')}</span>
        </a>
        <a class="nav-item { page.url.pathname === '/check-passwords' ? 'active' : '' }" href="/check-passwords">
          <img class="icon" src="/icons/shield.svg" alt="Check Passwords icon" />
          <span>{$t('check_passwords')}</span>
        </a>
      </nav>
      <button class="settings-btn" onclick={() => showSettings = true} aria-label="Open settings">
        <img class="icon" src="/icons/gear.svg" alt={$t('settings')} />
        <span>{$t('settings')}</span>
      </button>
      <button
        class="toggle-btn"
        style={`left: ${isSidebarCollapsed ? '80px' : '240px'};`}
        onclick={toggleSidebar}
        aria-label="Toggle sidebar"
      >
        <img
          class="arrow"
          class:collapsed={isSidebarCollapsed}
          src="https://api.iconify.design/mdi:chevron-left.svg"
          alt="toggle sidebar"
        />
      </button>
    </aside>
    <main class="content">
      <div class="content-wrapper" style:visibility={isLoading ? 'hidden' : 'visible'}>
        {@render children?.()}
      </div>
      {#if isLoading}
        <div class="loading-overlay">
          <div class="loading-bar"></div>
        </div>
      {/if}
    </main>
  </div>

  {#if showSettings}
    <div class="modal-backdrop" onclick={(e) => { if (e.target === e.currentTarget) showSettings = false; }} onkeydown={(e) => e.key === 'Escape' && (showSettings = false)} role="dialog" aria-modal="true" aria-labelledby="dialog-title" tabindex="-1">
      <div class="modal-content" role="document">
        <h2 id="dialog-title">{$t('settings')}</h2>

        <div class="setting-item">
          <label for="theme-switcher">{$t('theme')}</label>
          <div class="theme-switcher">
            <button class:active={currentTheme === 'light'} onclick={() => toggleTheme('light')}>{$t('light')}</button>
            <button class:active={currentTheme === 'dark'} onclick={() => toggleTheme('dark')}>{$t('dark')}</button>
          </div>
        </div>

        <div class="setting-item">
          <label for="select_language">{$t('select_language')}</label>
          <div class="language-select" aria-label="{$t('select_language')}">
            <button
              class:active={$language === 'en'}
              onclick={() => setLanguage('en')}
              aria-label="Select English"
            >
              <img src="/icons/english_flag.svg" alt="English Flag" class="flag-icon" />
              English
            </button>
            <button
              class:active={$language === 'ro'}
              onclick={() => setLanguage('ro')}
              aria-label="Select Romanian"
            >
              <img src="/icons/romanian_flag.svg" alt="Romanian Flag" class="flag-icon" />
              Română
            </button>
          </div>
        </div>

        <div class="setting-item">
          <label for="license-key">{$t('license_key')}</label>
          <div class="license-key-wrapper">
            <input type="text" id="license-key" placeholder={$t('enter_your_license_key')} bind:value={$licenseKey} />
            <button class="activate-btn" onclick={activateLicense}>{$t('activate')}</button>
          </div>
          {#if activationAttempted}
            <div class="server-ping-loader" class:success={pingSuccess}>
              <div class="progress-bar" style="width: {pingProgress}%;"></div>
            </div>
          {/if}
        </div>

        <!-- <div class="setting-item">
          <label for="auto-lock">{$t('auto_lock_after_inactivity')}</label>
          <div class="auto-lock-setting">
            <input type="checkbox" id="auto-lock" bind:checked={autoLockEnabled} />
            {#if autoLockEnabled}
              <input type="number" min="1" bind:value={autoLockTime} />
              <span>{$t('minutes')}</span>
            {/if}
          </div>
        </div> -->

        <button class="close-modal" onclick={() => showSettings = false}>{$t('close')}</button>
      </div>
    </div>
  {/if}
{:else}
  <Welcome />
{/if}

<style>
  @font-face {
    font-family: 'Lexend';
    src: url('/fonts/Lexend.ttf') format('truetype');
  }

  :global(html, body) {
    user-select: none;
    margin: 0;
    height: 100%;
    font-family: Lexend, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    box-sizing: border-box;
  }

  :global(*, *::before, *::after) {
    box-sizing: inherit;
  }

  :global(body.light-mode-filter) {
    filter: invert(1);
  }

  :global(body.light-mode-filter img) {
    filter: invert(1);
  }

  :global(body.light-mode-filter .last-change) {
    filter: invert(1);
  }

  :global(body.light-mode-filter .chart-container canvas) {
    filter: invert(1);
  }

  :global(body.light-mode-filter .toolbar button) {
    filter: invert(1);
  }

  :global(body.light-mode-filter .status-icon) {
    filter: invert(1);
  }

  :global(body.light-mode-filter .browser-selector) {
    filter: invert(1);
  }

  :global(body.light-mode-filter .close-modal) {
    filter: invert(1);
  }

  :global(body.light-mode-filter .setting-item) {
    filter: invert(1);
  }

  :global(body.light-mode-filter .setting-item img) {
    filter: invert(0);
  }

  :global(body.light-mode-filter .sort-dropdown) {
    color: #1a1a1a;
  }
  :global(body.light-mode-filter .sort-dropdown .active) {
    color: #e5e5e5
  }
  :global(body.light-mode-filter .sort-dropdown .sort-option:hover) {
    color: #e5e5e5
  }

  .language-select {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
  }

  .language-select button {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid #444;
    border-radius: 8px;
    background: rgba(68, 68, 68, 0.2);
    color: #e0e0e0;
    cursor: pointer;
    font-weight: 500;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    transition: all 0.2s ease;
    font-family: 'Lexend', sans-serif;
  }

  .language-select button:hover {
    background: rgba(68, 68, 68, 0.4);
  }

  .language-select button.active {
    background: #4caf50;
    border-color: #4caf50;
    color: white;
  }

  .flag-icon {
    width: 24px;
    height: 16px;
    border-radius: 2px;
    object-fit: cover;
  }

  .app-container {
    display: flex;
    height: 100vh;
    background: #0d0d0d;
    color: #e0e0e0;
    overflow: hidden;
  }

  .sidebar {
    width: 240px;
    background: #191919;
    display: flex;
    flex-direction: column;
    padding: 0 0.75rem 0.75rem 0.75rem;
    transition: width 0.3s ease;
  }

  .sidebar.collapsed {
    width: 80px;
  }

  .toggle-btn {
    position: absolute;
    left: 19.4rem;
    top: 50%;
    transform: translate(-50%, -50%);
    transition: left 0.3s ease;
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: #191919;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    cursor: pointer;
    z-index: 10;
  }

  .arrow {
    width: 1.5rem;
    height: 1.5rem;
    filter: invert(1);
    transition: transform 0.3s ease;
  }

  .arrow.collapsed {
    transform: rotate(180deg);
  }

  .nav-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 0;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: transparent;
    border: none;
    color: inherit;
    font-size: 1.05rem;
    font-weight: 600;
    padding: 1rem;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.2s;
    text-decoration: none;
  }

  .nav-list .nav-item:first-child {
    padding: 0 1rem 1rem 1rem;
  }

  .nav-item:hover {
    background: #3a3a3a;
  }

  .nav-item.active {
    background: #4a4a4a;
  }

  .icon {
    width: 1.25rem;
    height: 1.25rem;
    flex-shrink: 0;
    filter: brightness(0) invert(1);
  }

  .sidebar.collapsed .nav-item span,
  .sidebar.collapsed .settings-btn span {
    display: none;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: 1rem;
  }

  .sidebar.collapsed .icon {
    margin: 0;
  }

  .settings-btn {
    color: white;
    background: transparent;
    border: none;
    padding: 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: left;
    gap: 0.75rem;
    font-size: 1rem;
    font-weight: 600;
    transition: background 0.2s;
    margin-top: auto;
    text-decoration: none;
        font-family: 'Lexend', sans-serif;
  }

  .settings-btn:hover {
    background: #3a3a3a;
  }

  .content {
    flex: 1;
    padding: 1.5rem;
    background: #0d0d0d;
    overflow-y: auto;
    position: relative;
  }

  .loading-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: #0d0d0d;
    display: flex;
    justify-content: top;
    align-items: top;
    z-index: 9000;
    padding-top: 1%;
  }

  .loading-bar {
    position: relative;
    width: 100%;
    max-width: 4000px;
    height: 6px;
    background: #4a4a4a;
    overflow: hidden;
  }

  .loading-bar::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 30%;
    height: 100%;
    background: #e0e0e0;
    animation: loading 1.5s ease-in-out infinite;
  }

  @keyframes loading {
    0% { transform: translateX(-100%); }
    50% { transform: translateX(300%); }
    100% { transform: translateX(300%); }
  }

  .content-wrapper {
    animation: fadeIn 0.3s ease-in;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes fadeInElement {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .titlebar {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 10px;
    overflow: hidden;
    transition: height 0.3s ease-in-out;
    background: black;
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.25rem;
    border-bottom-right-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .titlebar.expanded {
    height: 2.0rem;
  }

  .titlebar-button {
    background: black;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    outline: none;
    opacity: 0;
    transition: opacity 0.3s ease-in-out;
  }

  .titlebar.expanded .titlebar-button {
    opacity: 1;
  }

  .titlebar-button img {
    width: 1.5rem;
    height: 1.5rem;
    filter: invert(1);
  }

  .welcome-icon {
    width: 1px;
    height: auto;
    transition: width 1s ease-in-out;
    align-self: center;
    filter: invert(1);
    opacity: 0%;
  }

  .sidebar.collapsed .welcome-icon {
    width: 80px;
    filter: invert(1);
    opacity: 100%;
    transition: width 1s ease-in-out;
  }

  .no-click {
    pointer-events: none;
  }

  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1001;
  }

  .modal-content {
    background: rgba(30, 30, 30, 0.85);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(68, 68, 68, 0.6);
    padding: 3rem;
    border-radius: 12px;
    width: 90%;
    max-width: 500px;
    display: flex;
    justify-content: center;
    flex-direction: column;
    gap: 1.5rem;
    box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);
    font-family: 'Lexend', sans-serif;
  }

  .modal-content h2 {
    margin: 0;
    font-size: 1.8rem;
    font-weight: 700;
    color: #e0e0e0;
    font-family: 'Lexend', sans-serif;
    display: flex;
    justify-content: center;
  }

  .setting-item label {
    display: flex;
    justify-content: center;
    font-weight: 500;
    color: #e0e0e0;
    font-family: 'Lexend', sans-serif;
    padding-bottom: 3%;
  }

  .theme-switcher {
    display: flex;
    gap: 0.5rem;
  }

  .theme-switcher button {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid #444;
    border-radius: 8px;
    background: rgba(68, 68, 68, 0.2);
    color: #e0e0e0;
    cursor: pointer;
    transition: all 0.2s ease;
    font-family: 'Lexend', sans-serif;
  }

  .theme-switcher button:hover {
    background: rgba(68, 68, 68, 0.4);
  }

  .theme-switcher button.active {
    background: #4caf50;
    border-color: #4caf50;
    color: white;
  }

  /* .auto-lock-setting input[type="checkbox"] {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    width: 1.5rem;
    height: 1.5rem;
    border: 2px solid #4caf50;
    border-radius: 4px;
    background-color: rgba(68, 68, 68, 0.2);
    cursor: pointer;
    position: relative;
    outline: none;
    transition: all 0.2s ease;
  }

  .auto-lock-setting input[type="checkbox"]:checked {
    background-color: #4caf50;
    border-color: #4caf50;
  }

  .auto-lock-setting input[type="checkbox"]:checked::after {
    content: '';
    position: absolute;
    top: 3px;
    left: 5px;
    width: 6px;
    height: 12px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
  }

  .auto-lock-setting input[type="number"] {
    width: 60px;
    padding: 0.5rem;
    border-radius: 6px;
    border: 1px solid #444;
    background: rgba(68, 68, 68, 0.2);
    color: #e0e0e0;
    font-family: 'Lexend', sans-serif;
  } */

  .close-modal {
    align-self: flex;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 8px;
    background: #4caf50;
    color: white;
    cursor: pointer;
    font-weight: 600;
    font-family: 'Lexend', sans-serif;
    transform: translate(0%, +50%);
  }

  .license-key-wrapper {
    display: flex;
    gap: 0.5rem;
  }

  .license-key-wrapper input {
    flex: 1;
    padding: 0.75rem;
    border-radius: 8px;
    border: 1px solid #444;
    background: rgba(68, 68, 68, 0.2);
    color: #e0e0e0;
    font-family: 'Lexend', sans-serif;
  }

  .activate-btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 8px;
    background: #4caf50;
    color: white;
    cursor: pointer;
    font-weight: 600;
    font-family: 'Lexend', sans-serif;
  }

  .server-ping-loader {
    width: 100%;
    height: 4px;
    background-color: #444;
    margin-top: 0.5rem;
    border-radius: 2px;
    position: relative;
    overflow: hidden;
  }

  .progress-bar {
    height: 100%;
    background-color: #f44336;
    transition: width 0.3s ease-in-out;
  }

  .server-ping-loader.success .progress-bar {
    background-color: #4caf50;
  }
</style>
