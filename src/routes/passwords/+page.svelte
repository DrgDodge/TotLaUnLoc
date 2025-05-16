<script lang="ts">
  import { onMount } from 'svelte';
  import { writable, derived } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';

  interface Entry {
    id: number;
    icon: string;
    account: string;
    username: string;
    date_created: string;
    date_modified: string;
  }

  // Domain to account name mapping
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

  // Function to extract domain from URL
  function getDomain(url: string): string {
    try {
      const { hostname } = new URL(url);
      return hostname
        .replace(/^www\./, '')
        .split('.')
        .slice(-2)
        .join('.');
    } catch (e) {
      return '';
    }
  }

  const search = writable('');
  const entries = writable<Entry[]>([]);
  const sortKey = writable<keyof Entry>('account');
  const sortAsc = writable(true);
  let status = writable<'loading' | 'success' | 'empty' | 'error'>('loading');
  let revealLoading = writable<number | null>(null);

  const filtered = derived(
    [entries, search, sortKey, sortAsc],
    ([$entries, $search, $sortKey, $sortAsc]) => {
      const filteredEntries = $entries.filter(e =>
        e.account.toLowerCase().includes($search.toLowerCase())
      );
      return filteredEntries.sort((a, b) => {
        const aVal = a[$sortKey];
        const bVal = b[$sortKey];
        const order = $sortAsc ? 1 : -1;
        if (typeof aVal === 'string') {
          return aVal.localeCompare(bVal as string) * order;
        }
        return (aVal > bVal ? 1 : -1) * order;
      });
    }
  );

  // Fetch and process data
  onMount(async () => {
    try {
      const jsonData: string = await invoke('passwords');
      const rawPasswords  = JSON.parse(jsonData);

      
      const entriesData = rawPasswords.map((p: any, index: number) => {
        const domain = getDomain(p.url);
        const account = domain ? (websiteAccounts[domain] || domain) : 'Unknown';
        const icon = domain ? `https://${domain}/favicon.ico` : '/icons/default.svg';

        return {
          id: index + 1,
          icon,
          account,
          username: p.username,
          date_created: p.date_created,
          date_modified: p.date_modified,
        };
      });

      entries.set(entriesData);
      status.set(entriesData.length ? 'success' : 'empty');
    } catch (error) {
      console.error('Failed to load passwords:', error);
      status.set('error');
    }
  });

  function toggleSort(key: keyof Entry) {
    sortKey.update($sortKey => {
      if ($sortKey === key) {
        sortAsc.update($asc => !$asc);
        return key;
      }
      sortAsc.set(true);
      return key;
    });
  }

  async function reveal(id: number) {
    revealLoading.set(id);
    try {
      // const password = await invoke('get_password', { id });
      // Handle password reveal logic
    } catch (error) {
      console.error('Password reveal failed:', error);
    }
    revealLoading.set(null);
  }
</script>

<div class="page-wrapper">
  <!-- Search Bar -->
  <div class="toolbar">
    <div class="search-wrapper">
      <img class="search-icon" src="/icons/search.svg" alt="Search" />
      <input
        type="text"
        placeholder="Search"
        bind:value={$search}
        aria-label="Search accounts"
      />
    </div>
  </div>

  <!-- Status Messages -->
  {#if $status === 'loading'}
    <div class="status-message">Loading...</div>
  {:else if $status === 'empty'}
    <div class="status-message">No passwords found.</div>
  {:else if $status === 'error'}
    <div class="status-message error">Failed to load password data.</div>
  {:else}
    <!-- Password Table -->
    <div class="table-wrapper">
      <table class="entries" role="grid">
        <thead>
          <tr>
            <th on:click={() => toggleSort('account')}
                class:active={$sortKey === 'account'}
                class:asc={$sortAsc}
                aria-sort={$sortKey === 'account' ? ($sortAsc ? 'ascending' : 'descending') : 'none'}>
              Account
            </th>
            <th on:click={() => toggleSort('username')}
                class:active={$sortKey === 'username'}
                class:asc={$sortAsc}
                aria-sort={$sortKey === 'username' ? ($sortAsc ? 'ascending' : 'descending') : 'none'}>
              Username
            </th>
            <th on:click={() => toggleSort('date_created')}
                class:active={$sortKey === 'date_created'}
                class:asc={$sortAsc}
                aria-sort={$sortKey === 'date_created' ? ($sortAsc ? 'ascending' : 'descending') : 'none'}>
              Date Created
            </th>
            <th on:click={() => toggleSort('date_modified')}
                class:active={$sortKey === 'date_modified'}
                class:asc={$sortAsc}
                aria-sort={$sortKey === 'date_modified' ? ($sortAsc ? 'ascending' : 'descending') : 'none'}>
              Date Modified
            </th>
            <th>Password</th>
            <th aria-hidden="true"></th>
          </tr>
        </thead>
        <tbody>
          {#each $filtered as e}
            <tr role="row">
              <td class="account-cell" role="gridcell">
                <img src={e.icon} alt={e.account} on:error={(e) => e.target.src = '/icons/default.svg'} />
                {e.account}
              </td>
              <td role="gridcell">{e.username}</td>
              <td role="gridcell">{new Date(e.date_created).toLocaleDateString()}</td>
              <td role="gridcell">{new Date(e.date_modified).toLocaleDateString()}</td>
              <td role="gridcell">••••••••</td>
              <td role="gridcell">
                <button class="eye-btn"
                        on:click={() => reveal(e.id)}
                        disabled={$revealLoading === e.id}
                        aria-label="Reveal password for {e.account}">
                  <img class="eye-icon" src="/icons/eye.svg" alt="" />
                </button>
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