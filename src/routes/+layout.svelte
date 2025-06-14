<script lang="ts">
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";
  import SidebarBak from "$lib/components/elements/sidebar.bak.svelte";
  import WindowBar from "$lib/components/elements/window-bar.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/elements/sidebar.svelte";

  let { children } = $props();

</script>

<ModeWatcher />
<Sidebar.Provider>
  <AppSidebar />
  <main>
    <Sidebar.Trigger />
    {@render children?.()}
  </main>
</Sidebar.Provider>


<!-- <div class="flex h-screen bg-black text-gray-300 overflow-hidden"> -->
  <!-- <WindowBar /> -->
  <!-- <SidebarBak /> -->
  <!-- {@render children?.()} -->
  <!-- <main class="content">
    {#if isLoading}
      <div class="loading-bar"></div>
    {:else}
      <div class="content-wrapper">
        <slot />
      </div>
    {/if}
  </main> -->
<!-- </div> -->