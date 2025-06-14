<script lang="ts">
  import { toggleMode } from "mode-watcher";

  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import Button from "$lib/components/ui/button/button.svelte";

  import KeyRoundIcon from "@lucide/svelte/icons/key-round";
  import ClockFadingIcon from "@lucide/svelte/icons/clock-fading";
  import ShieldUserIcon from "@lucide/svelte/icons/shield-user";
  import SunIcon from "@lucide/svelte/icons/sun";
  import MoonIcon from "@lucide/svelte/icons/moon";

  // Menu items.
  const items = [
    {
      title: "Passwords",
      url: "/passwords",
      icon: KeyRoundIcon,
    },
    {
      title: "OTP",
      url: "/one-time-codes",
      icon: ClockFadingIcon,
    },
    {
      title: "Check Passwords",
      url: "/check-passwords",
      icon: ShieldUserIcon,
    },
  ];
</script>

<Sidebar.Root collapsible="icon">
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each items as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                {#snippet child({ props })}
                  <a href={item.url} {...props}>
                    <item.icon />
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton onclick={toggleMode}>
          <SunIcon
            class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
          />
          <MoonIcon
            class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
          />
          <span>Toggle Theme</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Footer>
</Sidebar.Root>
