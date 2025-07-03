<script lang="ts">
  import "../app.css";

  import { page } from "$app/stores";
  import { beforeNavigate } from "$app/navigation";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  import { toggleMode } from "mode-watcher";

  let isLoading = $state(false);
  let loadingTimeout: NodeJS.Timeout | null = null;
  let isDragging = $state(false);
  let isMouseOver = $state(false);
  let isSidebarCollapsed = $state(false);

  let { children } = $props();
  let isExpanded = $derived(isDragging || isMouseOver);

  const getRandomDelay = () => Math.random() * (800 - 200) + 200;

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

  onMount(() => {
    isLoading = false;
    return () => {
      if (loadingTimeout) clearTimeout(loadingTimeout);
    };
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

  function handleMouseDown(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (target.closest("button")) return;
    isDragging = true;
    startDrag();
    const handleMouseUp = () => {
      isDragging = false;
      window.removeEventListener("mouseup", handleMouseUp);
    };
    window.addEventListener("mouseup", handleMouseUp);
  }
</script>

<div
  class="titlebar"
  class:expanded={isExpanded}
  onmouseenter={() => (isMouseOver = true)}
  onmouseleave={() => (isMouseOver = false)}
  role="button"
  tabindex="0"
  onmousedown={handleMouseDown}
>
  <button
    id="titlebar-minimize"
    class="cursor-pointer p-1 flex items-center justify-center opacity-0"
    onclick={minimize}
  >
    <img
      alt="minimize"
      src="https://api.iconify.design/mdi:window-minimize.svg"
    />
  </button>
  <button
    id="titlebar-close"
    class="cursor-pointer p-1 flex items-center justify-center opacity-0"
    onclick={close}
  >
    <img alt="close" src="https://api.iconify.design/mdi:close.svg" />
  </button>
</div>
<div class="flex h-screen text-neutral-200 overflow-hidden">
  <aside class:w-60={!isSidebarCollapsed} class:w-20={isSidebarCollapsed} class="flex flex-col p-3 transition-all">
    <button
      aria-label="Toggle sidebar"
      class="absolute left-80 top-1/2 translate-y-1/4 w-10 h-10 flex items-center justify-center cursor-pointer z-10"
      onclick={toggleSidebar}
    >
      <img
        alt="toggle sidebar"
        src="https://api.iconify.design/mdi:chevron-left.svg"
        class:rotate-180={isSidebarCollapsed}
        class="w-6 h-6 transition-transform"
      />
    </button>
    <nav class="pt-6 flex flex-col gap-3">
      <a
        href="/passwords"
        class="flex items-center gap-3 text-inherit text-base font-semibold p-4 rounded-md cursor-pointer {$page
          .url.pathname === '/passwords'
          ? 'active'
          : ''}"
      >
        <img
          alt="Passwords icon"
          src="/icons/key.svg"
          class="w-5 h-5 shrink-0 brightness-100"
        />
        <span class:hidden={isSidebarCollapsed}>Passwords</span>
      </a>
      <a
        href="/one-time-codes"
        class="flex items-center gap-3 text-inherit text-base font-semibold p-4 rounded-md cursor-pointer {$page
          .url.pathname === '/one-time-codes'
          ? 'active'
          : ''}"
      >
        <img
          alt="One-Time Codes icon"
          src="/icons/clock.svg"
          class="w-5 h-5 shrink-0 brightness-100"
        />
        <span class:hidden={isSidebarCollapsed}>One-Time Codes</span>
      </a>
      <a
        href="/check-passwords"
        class="flex items-center gap-3 text-inherit text-base font-semibold p-4 rounded-md cursor-pointer {$page
          .url.pathname === '/check-passwords'
          ? 'active'
          : ''}"
      >
        <img
          alt="Check Passwords icon"
          src="/icons/shield.svg"
          class="w-5 h-5 shrink-0 brightness-100"
        />
        <span class:hidden={isSidebarCollapsed}>Check Passwords</span>
      </a>
    </nav>
    <button
      aria-label="Change the theme of the app"
      class="text-white p-3 rounded-md cursor-pointer flex items-center gap-3 text-base font-semibold mt-auto"
      onclick={toggleMode}
    >
      <img
        alt="Change theme"
        src="/icons/theme.svg"
        class="w-6 h-6 brightness-100"
      />
      <span class:hidden={isSidebarCollapsed}>Change Theme</span>
    </button>
  </aside>
  <main class="flex-1 p-6 overflow-y-auto relative">
    {#if isLoading}
      <div class="absolute top-0 left-0 w-full h-1 overflow-hidden">
        <div class="loading-bar-indeterminate"></div>
      </div>
    {/if}
    {@render children()}
  </main>
</div>

<style>
  @font-face {
    font-family: "Lexend";
    src: url("/fonts/Lexend.ttf") format("truetype");
  }

  :global(html, body) {
    user-select: none;
    margin: 0;
    height: 100%;
    font-family:
      Lexend,
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      Roboto,
      Helvetica,
      Arial,
      sans-serif;
  }

  .loading-bar-indeterminate {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 4px;
    background: #4a4a4a;
    overflow: hidden;
  }

  .loading-bar-indeterminate::before {
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
    0% {
      transform: translateX(-100%);
    }
    50% {
      transform: translateX(300%);
    }
    100% {
      transform: translateX(300%);
    }
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
    height: 2rem;
  }

  
</style>
