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