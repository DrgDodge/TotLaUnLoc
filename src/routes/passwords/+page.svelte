<script lang="ts">
  import { onMount } from 'svelte';
  import { writable, derived } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';




  interface Entry {
    id: number;
    icon: string;
    account: string;
    username: string;
    url: string;
    lastChangeDays: number;
  }

  const websiteAccounts: { [key: string]: string } = {
    'google.com': 'Google',
    'facebook.com': 'Facebook',
    'dropbox.com': 'Dropbox',
    'twitter.com': 'Twitter',
    'linkedin.com': 'LinkedIn',
    'github.com': 'GitHub',
    'microsoft.com': 'Microsoft',
    'amazon.com': 'Amazon',
  };

  function getDomain(url: string): string {
    try {
      const { hostname } = new URL(url);
      return hostname.replace(/^www\./, '').split('.').slice(-2).join('.');
    } catch {
      return '';
    }
  }

  const search = writable('');
  const entries = writable<Entry[]>([]);
  const sortKey = writable<keyof Entry>('account');
  const sortAsc = writable(true);
  let status = writable<'loading' | 'success' | 'empty' | 'error'>('loading');

  const filtered = derived(
    [entries, search, sortKey, sortAsc],
    ([$entries, $search, $sortKey, $sortAsc]) => {
      return $entries
        .filter(e => e.account.toLowerCase().includes($search.toLowerCase()))
        .sort((a, b) => {
          const aVal = a[$sortKey];
          const bVal = b[$sortKey];
          const order = $sortAsc ? 1 : -1;
          if (typeof aVal === 'string') return aVal.localeCompare(bVal as string) * order;
          return (aVal > bVal ? 1 : -1) * order;
        });
    }
  );

  onMount(async () => {
    try {
      const jsonData: string = await invoke('passwords');
      const raw = JSON.parse(jsonData);
      const now = Date.now();
      const data: Entry[] = raw.map((p: any, idx: number) => {
        const domain = getDomain(p.url);
        const account = domain ? (websiteAccounts[domain] || domain) : 'Unknown';
        const icon = domain ? `https://${domain}/favicon.ico` : '/icons/default.svg';
        const modified = new Date(p.date_modified).getTime();
        const diffDays = Math.floor((now - modified) / (1000 * 60 * 60 * 24));
        return { id: idx + 1, icon, account, username: p.username, url: p.url, lastChangeDays: diffDays };
      });
      entries.set(data);
      status.set(data.length ? 'success' : 'empty');
    } catch (e) {
      console.error(e);
      status.set('error');
    }
  });

  function toggleSort(key: keyof Entry) {
    sortKey.update(k => (k === key ? key : key));
    sortAsc.update((asc) => (sortKey === key ? !asc : true));
  }
</script>

<div class="page-wrapper">
  <div class="toolbar">
    <div class="search-wrapper">
      <img class="search-icon" src="/icons/search.svg" alt="" />
      <input type="text" placeholder="Search" bind:value={$search} aria-label="Search accounts" />
    </div>
  </div>

  {#if $status === 'loading'}
    <div class="status-message">Loading...</div>
  {:else if $status === 'empty'}
    <div class="status-message">No passwords found.</div>
  {:else if $status === 'error'}
    <div class="status-message error">Failed to load passwords.</div>
  {:else}
    <div class="table-wrapper">
      <table class="entries" role="grid">
        <thead>
          <tr>
            <th on:click={() => toggleSort('account')}>Account</th>
            <th on:click={() => toggleSort('username')}>Username</th>
            <th on:click={() => toggleSort('lastChangeDays')}>Last password change</th>
            <th>Manage</th>
          </tr>
        </thead>
        <tbody>
          {#each $filtered as e}
            <tr>
              <td class="account-cell">
                <img src={e.icon} alt={e.account} on:error={(e) => e.target.src = '/icons/default.svg'} />
                {e.account}
              </td>
              <td>{e.username}</td>
              <td class="last-change" class:recent={e.lastChangeDays < 180} class:moderate={e.lastChangeDays < 365 && e.lastChangeDays >= 180} class:stale={e.lastChangeDays >= 365}>
                {#if e.lastChangeDays < 30}
                  {e.lastChangeDays} days ago
                {:else if e.lastChangeDays < 365}
                  {Math.floor(e.lastChangeDays / 30)} months ago
                {:else}
                  {Math.floor(e.lastChangeDays / 365)} years ago
                {/if}
              </td>
              <td>
                <a href={e.url} target="_blank" rel="noopener" class="manage-btn" aria-label="Manage {e.account}">
                  <img src="/icons/arrow.svg" alt="Go" />
                </a>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  :global(:root) {
    --panel: #141414;
    --text: #e0e0e0;
    --muted: #777;
    --hover: #272727;
    --border: #444;
  }
  .page-wrapper {
    display: flex;
    flex-direction: column;
    height: 90vh;
    overflow: hidden;
    background: var(--panel);
  }
  .toolbar {
    padding: 1rem 0;
    background: rgb(13, 13, 13);
    flex: none;
    display: flex;
    align-items: center;
  }
  .search-wrapper {
    padding-top: 0rem;
    flex: 1;
    position: relative;
    min-width: 200px;
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
    filter: invert(1);
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

  .table-wrapper {
    flex: 1;
    overflow-y: auto;
  }
  .entries {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;
  }
  thead {
    background: var(--panel);
    position: sticky;
    top: 0;
    z-index: 1;
    border-bottom: 1px solid var(--border);
  }
  thead th {
    padding: 1rem;
    text-align: left;
    font-weight: 700;
    font-size: 1.1rem;
    color: var(--text);
    cursor: pointer;
  }
  th:nth-child(1), td:nth-child(1) { width: 35%; }
  th:nth-child(2), td:nth-child(2) { width: 25%; }
  th:nth-child(3), td:nth-child(3) { width: 25%; }
  th:nth-child(4), td:nth-child(4) { width: 15%; text-align: center; }

  tbody tr:hover { background: var(--hover); }
  tbody td { padding: 0.75rem 1rem; vertical-align: middle; }

  .account-cell { display: flex; align-items: center; gap: 0.75rem; }
  .account-cell img { width: 1.75rem; height: 1.75rem; object-fit: contain; background: white; border-radius: 4px; }

  .last-change.recent { color: #4caf50; }
  .last-change.moderate { color: #ffeb3b; }
  .last-change.stale { color: #f44336; }

  .manage-btn { background: transparent; padding: 0.25rem; border-radius: 6px; display: inline-flex; align-items: center; }
  .manage-btn img { width: 1.25rem; height: 1.25rem; filter: brightness(0) invert(1); }
  .manage-btn:hover { background: var(--hover); }

  /* Custom scrollbar */
  .table-wrapper::-webkit-scrollbar {
    width: 8px;
  }
  .table-wrapper::-webkit-scrollbar-track {
    background: var(--panel);
  }
  .table-wrapper::-webkit-scrollbar-thumb {
    background-color: var(--border);
    border-radius: 4px;
  }
  .table-wrapper::-webkit-scrollbar-thumb:hover {
    background-color: var(--text);
  }

  .status-message { padding: 1rem; color: var(--muted); text-align: center; }
  .status-message.error { color: #f44336; }

  
</style>