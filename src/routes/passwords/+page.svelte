<script lang="ts">
  import { onMount } from 'svelte';
  import { writable, derived } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';

  interface Entry {
    id: number;
    icon: string;
    account: string;
    username: string;
    date_created: string; // Adjusted to string for JSON compatibility
    date_modified: string; // Adjusted to string for JSON compatibility
  }

  // Mapping of domains to account names and icons
  const websiteIcons: { [key: string]: { account: string; icon: string } } = {
    'google.com': { account: 'Google', icon: '/icons/google.svg' },
    'facebook.com': { account: 'Facebook', icon: '/icons/facebook.svg' },
    'dropbox.com': { account: 'Dropbox', icon: '/icons/dropbox.svg' },
    'twitter.com': { account: 'Twitter', icon: '/icons/twitter.svg' },
    // Add more websites as needed
  };

  // Function to extract domain from URL
  function getDomain(url: string): string {
    try {
      const { hostname } = new URL(url);
      return hostname.replace(/^www\./, '');
    } catch (e) {
      return '';
    }
  }

  const search = writable('');
  const entries = writable<Entry[]>([]);

  const filtered = derived(
    [entries, search],
    ([$entries, $search]) =>
      $entries.filter(e =>
        e.account.toLowerCase().includes($search.toLowerCase())
      )
  );

  // Fetch data from Rust backend on mount
  onMount(async () => {
    try {
      const jsonData: string = await invoke('greet');
      const rawPasswords = JSON.parse(jsonData);
      const entriesData = rawPasswords.map((p: any, index: number) => {
        const domain = getDomain(p.url);
        const websiteInfo = websiteIcons[domain] || { account: domain || 'Unknown', icon: '/icons/default.svg' };
        return {
          id: index + 1,
          icon: websiteInfo.icon,
          account: websiteInfo.account,
          username: p.username,
          date_created: p.date_created,
          date_modified: p.date_modified,
        };
      });
      entries.set(entriesData);
    } catch (error) {
      console.error('Failed to fetch password data:', error);
    }
  });

  function reveal(id: number) {
    console.log('reveal', id);
    // Future implementation: Fetch and display the actual password
  }
</script>

<div class="page-wrapper">
  <!-- SEARCH BAR -->
  <div class="toolbar">
    <div class="search-wrapper">
      <img class="search-icon" src="/icons/search.svg" alt="" />
      <input
        type="text"
        placeholder="Search"
        bind:value={$search}
      />
    </div>
  </div>

  <!-- TABLE WITH FIXED HEADER -->
  <div class="table-wrapper">
    <table class="entries">
      <thead>
        <tr>
          <th>Account</th>
          <th>Username</th>
          <th>Date Created</th>
          <th>Date Modified</th>
          <th>Password</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each $filtered as e}
          <tr>
            <td class="account-cell">
              <img src={e.icon} alt="" />
              {e.account}
            </td>
            <td>{e.username}</td>
            <td>{new Date(e.date_created).toLocaleString()}</td>
            <td>{new Date(e.date_modified).toLocaleString()}</td>
            <td>••••••••</td>
            <td>
              <button class="eye-btn" on:click={() => reveal(e.id)}>
                <img class="eye-icon" src="/icons/eye.svg" alt="Reveal password" />
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  :global(:root) {
    --panel:  #141414;
    --text:   #e0e0e0;
    --muted:  #777;
    --hover:  #272727;
    --border: #444;
  }

  /* 1) Layout */
  .page-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  /* 2) Toolbar */
  .toolbar {
    padding-bottom: 1rem;
  }
  .search-wrapper {
    position: relative;
    width: 100%;
    box-sizing: border-box;
  }
  .search-icon {
    position: absolute;
    left: 1rem;
    top: 50%;
    width: 1.75rem;
    height: 1.75rem;
    transform: translateY(-50%);
    fill: var(--muted);
    pointer-events: none;
    flex-shrink: 0;
    filter: brightness(0) invert(1);
  }
  input {
    width: 100%;
    padding: 0.75rem 1rem 0.75rem 3rem;
    border: none;
    border-radius: 8px;
    background: var(--panel);
    color: var(--text);
    font-size: 1.1rem;
    box-sizing: border-box;
  }
  input::placeholder {
    color: var(--muted);
  }

  /* 3) Table wrapper: scrolls only rows */
  .table-wrapper {
    flex: 1;
    overflow-y: auto;
  }

  /* 4) Single table, fixed header */
  .entries {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;
    font-family: inherit;
  }
  thead {
    background: var(--panel);
    position: sticky;
    top: 0;
    z-index: 1;
    border-bottom: 1px solid var(--border);
  }
  thead th {
    padding: 1rem 1rem 0.75rem;
    text-align: left;
    font-weight: 700;
    font-size: 1.15rem;
    color: var(--text);
  }
  th:nth-child(1) { width: 20%; } /* Account */
  th:nth-child(2) { width: 20%; } /* Username */
  th:nth-child(3) { width: 20%; } /* Date Created */
  th:nth-child(4) { width: 20%; } /* Date Modified */
  th:nth-child(5) { width: 10%; } /* Password */
  th:nth-child(6) { width: 10%; } /* Button */

  /* 5) Body rows */
  tbody tr:hover {
    background: var(--hover);
  }
  tbody td {
    padding: 1rem;
    vertical-align: middle;
  }
  td:first-child  { padding-right: 2rem; }
  td:nth-child(2) { padding-right: 0.5rem; }
  td:nth-child(6) { text-align: center; } /* Center the button column */

  .account-cell {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  .account-cell img {
    width: 2rem;
    height: 2rem;
    object-fit: contain;
  }

  /* Eye button */
  .eye-btn {
    background: var(--panel);
    border: none;
    padding: 0.25rem;
    border-radius: 6px;
    cursor: pointer;
  }
  .eye-btn:hover {
    background: var(--hover);
  }
  .eye-icon {
    width: 1.5rem;
    height: 1.5rem;
    filter: brightness(0) invert(1);
  }

  /* Scrollbar styles */
  .table-wrapper::-webkit-scrollbar {
    width: 12px;
  }
  .table-wrapper::-webkit-scrollbar-track {
    background: #1a1a1a;
  }
  .table-wrapper::-webkit-scrollbar-thumb {
    background: #444;
    border-radius: 6px;
  }
  .table-wrapper::-webkit-scrollbar-thumb:hover {
    background: #666;
  }
</style>