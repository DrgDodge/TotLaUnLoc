<script lang="ts">
  import "../app.css";
  import { beforeNavigate } from "$app/navigation";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { ModeWatcher } from "mode-watcher";
  import Sidebar from "$lib/sidebar.svelte";

  let isLoading = false;
  let isDragging = false;
  let isMouseOver = false;
  let isExpanded = false;
  let isSidebarCollapsed = false;

  // $: isExpanded = isDragging || isMouseOver;

  const getRandomDelay = () => Math.random() * (800 - 200) + 200;

  function toggleTheme() {
    window.document.body.classList.toggle("dark-mode");
  }

  function toggleSidebar() {
    isSidebarCollapsed = !isSidebarCollapsed;
  }

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

<ModeWatcher />
<div class="app-container">
  <!-- <div
    class="titlebar"
    class:expanded={isExpanded}
    on:mouseenter={() => (isMouseOver = true)}
    on:mouseleave={() => (isMouseOver = false)}
    role="banner"
    on:mousedown={handleMouseDown}
  >
    <button class="titlebar-button" id="titlebar-minimize" on:click={minimize}>
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </button>
    <button class="titlebar-button" id="titlebar-close" on:click={close}>
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </button>
  </div> -->
  <!-- <Sidebar /> -->
  <main class="content">
    test
    {#if isLoading}
      <div class="loading-bar"></div>
    {:else}
      <div class="content-wrapper">
        <slot />
      </div>
    {/if}
  </main>
</div>

<!-- <style>
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

  :global(body.dark-mode) {
    filter: invert(1);
  }

  :global(body.dark-mode img) {
    filter: invert(1);
  }

  :global(body.dark-mode .last-change) {
    filter: invert(1);
  }

  .app-container {
    display: flex;
    height: 100vh;
    background: #0d0d0d;
    color: #e0e0e0;
    overflow: hidden;
  }

  .content {
    flex: 1;
    padding: 1.5rem;
    background: #0d0d0d;
    overflow-y: auto;
    position: relative;
  }

  .loading-bar {
    position: absolute;
    top: 10;
    left: 0;
    width: 100%;
    height: 4px;
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

  .content-wrapper {
    animation: fadeIn 0.3s ease-in;
  }

  /* .content-wrapper > * {
    opacity: 0;
    animation: fadeInElement 0.5s ease-in forwards;
  }

  .content-wrapper > *:nth-child(1) { animation-delay: 0s; }
  .content-wrapper > *:nth-child(2) { animation-delay: 0.1s; }
  .content-wrapper > *:nth-child(3) { animation-delay: 0.2s; }
  .content-wrapper > *:nth-child(4) { animation-delay: 0.3s; }
  .content-wrapper > *:nth-child(5) { animation-delay: 0.4s; }
  .content-wrapper > *:nth-child(6) { animation-delay: 0.5s; }
  .content-wrapper > *:nth-child(7) { animation-delay: 0.6s; }
  .content-wrapper > *:nth-child(8) { animation-delay: 0.7s; }
  .content-wrapper > *:nth-child(9) { animation-delay: 0.8s; }
  .content-wrapper > *:nth-child(10) { animation-delay: 0.9s; }
  .content-wrapper > *:nth-child(n + 11) { animation-delay: 1s; } */

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes fadeInElement {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
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
</style> -->
