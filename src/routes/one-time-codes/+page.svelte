<script lang="ts">
  import { writable, derived } from 'svelte/store';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  interface Entry {
    id: number;
    icon: string;
    account: string;
    username: string;
    code: string;
  }

  // Function to normalize account name into a domain
  function getDomain(account: string): string {
    // Convert account to lowercase and remove spaces
    const normalized = account.toLowerCase().replace(/\s+/g, '');
    // Try to parse as a URL if it looks like one
    try {
      const url = normalized.startsWith('http') ? normalized : `https://${normalized}`;
      const { hostname } = new URL(url);
      return hostname.replace(/^www\./, '');
    } catch (e) {
      // If not a valid URL, assume it's a domain-like string (e.g., "filelistt.io")
      return normalized;
    }
  }

  const entries = writable<Entry[]>([]);
  const search = writable('');
  let remaining = 30;

  const filtered = derived(
    [entries, search],
    ([$entries, $search]) =>
      $entries.filter(e =>
        e.account.toLowerCase().includes($search.toLowerCase())
      )
  );

  async function fetchCodes() {
    try {
      const accounts: Entry[] = await invoke('get_accounts_with_codes');
      // Process accounts to add dynamic favicon URLs
      const processedAccounts = accounts.map(account => {
        const domain = getDomain(account.account);
        const icon = `https://${domain}/favicon.ico`;
        return {
          ...account,
          icon,
        };
      });
      entries.set(processedAccounts);
      const now = Math.floor(Date.now() / 1000);
      remaining = 30 - (now % 30);
    } catch (error) {
      console.error('Failed to fetch codes:', error);
    }
  }

  async function addAccount() {
    const otpauth = prompt('Enter otpauth URL (e.g., otpauth://totp/user?secret=XXX&issuer=Service):');
    if (otpauth) {
      try {
        await invoke('add_account', { otpauth });
        await fetchCodes(); // Refresh codes after adding
      } catch (error) {
        alert('Failed to add account: ' + error);
      }
    }
  }

  async function deleteAccount(id: number) {
    try {
      await invoke('delete_account', { id });
      await fetchCodes(); // Refresh codes after deletion
    } catch (error) {
      alert('Failed to delete account: ' + error);
    }
  }

  onMount(() => {
    fetchCodes();
    const interval = setInterval(() => {
      remaining -= 1;
      if (remaining <= 0) {
        remaining = 30;
        fetchCodes();
      }
    }, 1000);
    return () => clearInterval(interval);
  });
</script>

<div class="page-wrapper">
  <div class="toolbar">
    <div class="search-wrapper">
      <img class="search-icon" src="/icons/search.svg" alt="" />
      <input
        type="text"
        placeholder="Search"
        bind:value={$search}
      />
    </div>
    <button class="add-button" on:click={addAccount}>
      <img src="/icons/add.svg" alt="Add Account" />
    </button>
    <div class="countdown-wrapper">
      <svg width="40" height="40" viewBox="0 0 40 40">
        <circle cx="20" cy="20" r="16" stroke="var(--muted)" stroke-width="4" fill="none" />
        <circle cx="20" cy="20" r="16" stroke="var(--text)" stroke-width="4" fill="none"
          stroke-dasharray="100.53" stroke-dashoffset={ (30 - remaining) / 30 * 100.53 } />
      </svg>
    </div>
  </div>
  <div class="table-wrapper">
    <table class="entries">
      <thead>
        <tr>
          <th>Account</th>
          <th>Username</th>
          <th>Code</th>
          <th aria-hidden="true"></th>
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
            <td class="code">{e.code.slice(0,3)} {e.code.slice(3)}</td>
            <td>
              <button class="delete-btn" on:click={() => deleteAccount(e.id)} aria-label="Delete account for {e.account}">
                <img class="delete-icon" src="/icons/trash.svg" alt="Delete" />
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

  .page-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding-bottom: 1rem;
  }

  .search-wrapper {
    flex: 1;
    position: relative;
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

  .add-button {
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: 50%;
    width: 40px;
    height: 40px;
    padding: 0;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.2s;
  }

  .add-button:hover {
    background: var(--hover);
  }

  .add-button img {
    width: 24px;
    height: 24px;
    filter: brightness(0) invert(1);
  }

  .countdown-wrapper {
    width: 40px;
    height: 40px;
  }

  .table-wrapper {
    flex: 1;
    overflow-y: auto;
  }

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

  th:nth-child(1) { width: 35%; } /* Account */
  th:nth-child(2) { width: 25%; } /* Username */
  th:nth-child(3) { width: 25%; } /* Code */
  th:nth-child(4) { width: 15%; } /* Delete button */

  tbody tr:hover {
    background: var(--hover);
  }

  tbody td {
    padding: 1rem;
    vertical-align: middle;
  }

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

  .code {
    font-family: monospace;
    font-size: 1.2rem;
    text-align: left;
  }

  .delete-btn {
    background: var(--panel);
    border: none;
    padding: 0.25rem;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .delete-btn:hover {
    background: var(--hover);
  }

  .delete-icon {
    width: 1.5rem;
    height: 1.5rem;
    filter: brightness(0) invert(1);
  }

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