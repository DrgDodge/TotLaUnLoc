<script lang="ts">
<<<<<<< HEAD
  import { page } from '$app/stores';
=======
    import { invoke } from "@tauri-apps/api/core";

  // no dynamic logic here

  function testRust() {
    invoke('greet')
  }
>>>>>>> 8d84560 (test-backend)
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
<<<<<<< HEAD
      </a>
      <a class="nav-item {($page.url.pathname === '/check-passwords' ? 'active' : '')}" href="/check-passwords">
        <img class="icon" src="/icons/shield.svg" alt="Check Passwords icon">
=======
      </button>
      <button class="nav-item" on:click={testRust}>
        <!-- Shield/check for Check Passwords -->
        <svg class="icon" viewBox="0 0 24 24">
          <path fill="currentColor"
            d="M12 1l9 4v6c0 5-3.7 10-9 11-5.3-1-9-6-9-11V5l9-4zm-1
               14l7-7-1.414-1.414L11 12.172l-2.586-2.586L7 11l4 4z"/>
        </svg>
>>>>>>> 8d84560 (test-backend)
        Check Passwords
      </a>
    </nav>

    <a class="settings {($page.url.pathname === '/settings' ? 'active' : '')}" href="/settings">
      <img class="icon" src="/icons/gear.svg" alt="Settings icon">
      Settings
    </a>
  </aside>

  <main class="content">
    <slot />
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
  }
</style>