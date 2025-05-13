<script lang="ts">
  import { writable, derived } from 'svelte/store';
  import { onMount } from 'svelte';

  function generateCode() {
    return Math.floor(100000 + Math.random() * 900000).toString();
  }

  interface Entry {
    id: number;
    icon: string;
    account: string;
    username: string; // Added username field
    code: string;
  }

  const initial: Entry[] = [
    { id: 1, icon: '/icons/google.svg', account: 'Google', username: 'user@gmail.com', code: generateCode() },
    { id: 2, icon: '/icons/facebook.svg', account: 'Facebook', username: 'user@fb.com', code: generateCode() },
    { id: 3, icon: '/icons/dropbox.svg', account: 'Dropbox', username: 'user@dropbox.com', code: generateCode() },
  ];

  const search = writable('');
  const entries = writable<Entry[]>(initial);

  const filtered = derived(
    [entries, search],
    ([$entries, $search]) =>
      $entries.filter(e =>
        e.account.toLowerCase().includes($search.toLowerCase())
      )
  );

  let remaining = 10;

  onMount(() => {
    const interval = setInterval(() => {
      remaining -= 1;
      if (remaining <= 0) {
        remaining = 10;
        entries.update(current => current.map(e => ({ ...e, code: generateCode() })));
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
    <div class="countdown-wrapper">
      <svg width="40" height="40" viewBox="0 0 40 40">
        <circle cx="20" cy="20" r="16" stroke="var(--muted)" stroke-width="4" fill="none" />
        <circle cx="20" cy="20" r="16" stroke="var(--text)" stroke-width="4" fill="none"
          stroke-dasharray="100.53" stroke-dashoffset={ (10 - remaining) / 10 * 100.53 } />
      </svg>
    </div>
  </div>
  <div class="table-wrapper">
    <table class="entries">
      <thead>
        <tr>
          <th>Account</th>
          <th>Username</th> <!-- Added Username column -->
          <th>Code</th>
        </tr>
      </thead>
      <tbody>
        {#each $filtered as e}
          <tr>
            <td class="account-cell">
              <img src={e.icon} alt="" />
              {e.account}
            </td>
            <td>{e.username}</td> <!-- Added username display -->
            <td class="code">{e.code.slice(0,3)} {e.code.slice(3)}</td>
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

  th:nth-child(1) { width: 40%; } /* Account */
  th:nth-child(2) { width: 30%; } /* Username */
  th:nth-child(3) { width: 30%; } /* Code */

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