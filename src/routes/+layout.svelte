<script lang="ts">
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";
  import SidebarBak from "$lib/components/elements/sidebar.bak.svelte";
  import WindowBar from "$lib/components/elements/window-bar.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/elements/sidebar.svelte";
  import Button from "$lib/components/ui/button/button.svelte";

  import XIcon from "@lucide/svelte/icons/x"
  import MinusIcon from "@lucide/svelte/icons/minus"

  let { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider>
  <AppSidebar />
  <main class="w-full">
    <div class="flex bg-sidebar">
      <Sidebar.Trigger />
      <Button variant="ghost" size="icon" class="size-7 ml-auto">
        <MinusIcon />
      </Button>
      <Button variant="ghost" size="icon" class="size-7">
        <XIcon />
      </Button>
    </div>
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
