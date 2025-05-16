<script lang="ts">
  import { page } from '$app/stores';
  import { beforeNavigate } from '$app/navigation';
  import { onMount } from 'svelte';

  let isLoading = false;
  let loadingTimeout: NodeJS.Timeout | null = null;

  // Function to generate random delay between 0.5s and 2s (in milliseconds)
  const getRandomDelay = () => {
    return Math.random() * (800 - 200) + 200; // Random value between 500ms and 2000ms
  };

  // Trigger loading state before navigation with random delay
  beforeNavigate(() => {
    // Clear any existing timeout to avoid overlap
    if (loadingTimeout) {
      clearTimeout(loadingTimeout);
    }
    
    isLoading = true;
    
    // Set random delay for loading bar
    const delay = getRandomDelay();
    loadingTimeout = setTimeout(() => {
      isLoading = false;
      loadingTimeout = null;
    }, delay);
  });

  // Ensure loading state is reset on initial mount
  onMount(() => {
    isLoading = false;
    return () => {
      // Cleanup timeout on component destroy
      if (loadingTimeout) {
        clearTimeout(loadingTimeout);
      }
    };
  });
</script>

<div class="app-container">
  <aside class="sidebar">
    <nav class="nav-list">
      <a class="nav-item {($page.url.pathname === '/passwords' ? 'active' : '')}" href="/passwords">
        <img class="icon" src="/icons/key.svg" alt="Passwords icon">
        Passwords
      </a>
      <a class="nav-item {($page.url.pathname === '/one-time-codes' ? 'active' : '')}" href="/one-time-codes">
        <img class="icon" src="/icons/clock.svg" alt="One-Time Codes icon">
        One-Time Codes
      </a>
      <a class="nav-item {($page.url.pathname === '/check-passwords' ? 'active' : '')}" href="/check-passwords">
        <img class="icon" src="/icons/shield.svg" alt="Check Passwords icon">
        Check Passwords
      </a>
    </nav>

    <a class="settings {($page.url.pathname === '/settings' ? 'active' : '')}" href="/settings">
      <img class="icon" src="/icons/gear.svg" alt="Settings icon">
      Settings
    </a>
  </aside>

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
  /* Global font stack */
  :global(html, body) {
    margin: 0;
    height: 100%;
    font-family: 'Roboto Mono', -apple-system, BlinkMacSystemFont,
                 "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
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
    padding: 0.75rem;
  }

  .nav-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding-top: 4.7rem; /* pushes tabs down */
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
  .nav-item:hover {
    background: #3a3a3a;
  }
  .nav-item.active, .settings.active {
    background: #4a4a4a;
  }

  .settings {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: transparent;
    border: none;
    color: inherit;
    font-size: 1.05rem;
    font-weight: 600;
    padding: 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.2s;
    margin-top: auto; /* push to bottom */
    text-decoration: none;
  }
  .settings:hover {
    background: #3a3a3a;
  }

  .icon {
    width: 1.25rem;
    height: 1.25rem;
    flex-shrink: 0;
    filter: brightness(0) invert(1);
  }

  .content {
    flex: 1;
    padding: 1.5rem;
    background: #0d0d0d;
    overflow-y: auto;
    position: relative; /* For positioning loading bar */
  }

  .loading-bar {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 4px;
    background: #4a4a4a;
    overflow: hidden;
  }

  .loading-bar::before {
    content: '';
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

  /* Apply staggered fade-in to direct children of content-wrapper */
  .content-wrapper > * {
    opacity: 0;
    animation: fadeInElement 0.5s ease-in forwards;
  }

  /* Stagger the animation delay for each child element */
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
  /* Fallback for additional elements */
  .content-wrapper > *:nth-child(n+11) { animation-delay: 1s; }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
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