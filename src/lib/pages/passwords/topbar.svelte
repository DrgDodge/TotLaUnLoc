<script lang="ts">
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Select from "$lib/components/ui/select/index.js";


  import ScanSearchIcon from "@lucide/svelte/icons/search"
</script>

<div class="toolbar">
  <!-- Search -->
  <div class="flex">

    <ScanSearchIcon class="size-9"/>
    <Input type="search" placeholder="Search" class="w-full"/>

    <!-- <div>test</div> -->

    <Select.Root type="single">
      <Select.Trigger class="w-[180px]"></Select.Trigger>
      <Select.Content>
        <Select.Item value="a-z">A - Z</Select.Item>
        <Select.Item value="z-a">Z - A</Select.Item>
      </Select.Content>
    </Select.Root>

  </div>

  <!-- <div class="search-wrapper">
      <img class="search-icon" src="/icons/search.svg" alt="" />
      <input
        type="text"
        placeholder="Search"
        bind:value={$search}
        aria-label="Search accounts"
      />
    </div> -->

  <!-- Sort Selector -->
  <!-- <div class="sort-selector">
      <button class="sort-button" on:click={() => showSortDropdown.update(v => !v)}>
        {sortOptions.find(o => o.key === $sortKey && o.asc === $sortAsc)?.label}
        <svg class="sort-arrow" viewBox="0 0 24 24"><path d="M7 10l5 5 5-5z" /></svg>
      </button>
      {#if $showSortDropdown}
        <div class="sort-dropdown" use:clickOutside>
          {#each sortOptions as option}
            <div
              class="sort-option {option.key === $sortKey && option.asc === $sortAsc ? 'active' : ''}"
              on:click={() => setSort(option.key, option.asc)}
            >
              {option.label}
            </div>
          {/each}
        </div>
      {/if}
    </div> -->

  <!-- Browser Selector -->
  <!-- <div class="browser-selector">
      <div class="selected-browser" on:click={togglePopup}>
        {#if $selectedBrowserName && browserIcons[$selectedBrowserName]}
          <img src={browserIcons[$selectedBrowserName]} alt={$selectedBrowserName} />
        {:else}
          <div class="placeholder"></div>
        {/if}
      </div>
      {#if showPopup}
        <div class="popup" use:clickOutside transition:fade>
          {#each $browserData as browser, i (browser.name)}
            <img
              src={browserIcons[browser.name] || '/icons/default.svg'}
              alt={browser.name}
              class:selected={browser.name === $selectedBrowserName}
              on:click={() => {
                selectedBrowserName.set(browser.name);
                showPopup = false;
              }}
              in:fly={{ y: -20, duration: 250, delay: i * 50 }}
              out:fly={{ y: 20, duration: 250 }}
              animate:flip
            />
          {/each}
        </div>
      {/if}
    </div> -->
</div>
