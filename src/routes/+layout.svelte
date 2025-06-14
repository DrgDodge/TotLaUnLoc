<script lang="ts">
  import "../app.css";
  import { beforeNavigate } from "$app/navigation";
  import { onMount } from "svelte";
  import { ModeWatcher } from "mode-watcher";
  import Sidebar from "$lib/sidebar.svelte";
    import WindowBar from "$lib/window-bar.svelte";

  let isLoading = false;
  let isDragging = false;
  let isMouseOver = false;
  let isExpanded = false;
  let isSidebarCollapsed = false;

  $: isExpanded = isDragging || isMouseOver;

  const getRandomDelay = () => Math.random() * (800 - 200) + 200;

  function toggleTheme() {
    window.document.body.classList.toggle("dark-mode");
  }

  function toggleSidebar() {
    isSidebarCollapsed = !isSidebarCollapsed;
  }
</script>

<ModeWatcher />
<div class="app-container">
  <WindowBar />
  <Sidebar />
  <main class="content">
    {#if isLoading}
      <div class="loading-bar"></div>
    {:else}
      <div class="content-wrapper">
        <slot />
      </div>
    {/if}
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
</style>
