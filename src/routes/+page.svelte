<script lang="ts">
  import { writable, derived } from 'svelte/store';

  interface Entry {
    id: number;
    icon: string;
    account: string;
    username: string;
  }

  const initial: Entry[] = [
    { id: 1, icon: '/icons/google.svg',   account: 'Google',   username: 'user@gmail.com'    },
    { id: 2, icon: '/icons/facebook.svg', account: 'Facebook', username: 'user@fb.com'      },
    { id: 3, icon: '/icons/dropbox.svg',  account: 'Dropbox',  username: 'user@dropbox.com' },
    { id: 4, icon: '/icons/twitter.svg',  account: 'Twitter',  username: 'user@twitter.com' }
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

  function reveal(id: number) {
    console.log('reveal', id);
  }
</script>

<div class="page-wrapper">
  <!-- SEARCH BAR -->
  <div class="toolbar">
    <div class="search-wrapper">
      <svg class="search-icon" viewBox="0 0 24 24">
        <path fill="currentColor"
          d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0
             16 9.5 6.5 6.5 0 1 0 9.5 16a6.471 6.471 0 0
             0 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6
             0C8.01 14 6 11.99 6 9.5S8.01 5 10.5 5 15 7.01 15
             9.5 12.99 14 10.5 14z"/>
      </svg>
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
            <td>••••••••</td>
            <td>
              <button class="eye-btn" on:click={() => reveal(e.id)}>
                <svg class="eye-icon" viewBox="0 0 24 24">
  <path fill="white" 
    d="M12 4.5C7.305 4.5 3.164 7.56 1.5 12c1.664 4.44 5.805 7.5 10.5 7.5s8.836-3.06 10.5-7.5C20.836 7.56 16.695 4.5 12 4.5zm0 13a5.5 5.5 0 1 1 0-11 5.5 5.5 0 0 1 0 11z"/>
  <path fill="currentColor" 
    d="M12 15a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7z"/>
</svg>
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
  th:nth-child(1) { width: 35%; } /* Account */
  th:nth-child(2) { width: 25%; } /* Username */
  th:nth-child(3) { width: 25%; } /* Password */
  th:nth-child(4) { width: 15%; } /* Button */

  /* 5) Body rows */
  tbody tr:hover {
    background: var(--hover);
  }
tbody td {
  padding: 1rem;
  vertical-align: middle; /* Changed from top to middle */
}
  /* Adjust spacing */
  td:first-child  { padding-right: 2rem; }
  td:nth-child(2) { padding-right: 0.5rem; }
  td:nth-child(4) { text-align: center; } /* Center the button column */

.account-cell {
  display: flex;
  align-items: center; /* Or flex-start, depending on your preference */
  gap: 1rem;
}
  .account-cell img {
    width: 2rem; /* Increased from 1.5rem */
    height: 2rem; /* Increased from 1.5rem */
    object-fit: contain;
  }

  /* Eye button */
  .eye-btn {
    background: var(--panel);
    border: none;
    padding: 0.25rem; /* Reduced from 0.5rem */
    border-radius: 6px;
    cursor: pointer;
  }
  .eye-btn:hover {
    background: var(--hover);
  }
  .eye-icon {
    width: 1.5rem;
    height: 1.5rem;
    fill: var(--text);
  }

  /* Scrollbar styles */
  .table-wrapper::-webkit-scrollbar {
    width: 12px;
  }
  .table-wrapper::-webkit-scrollbar-track {
    background: #1a1a1a; /* Very dark gray, nearly black */
  }
  .table-wrapper::-webkit-scrollbar-thumb {
    background: #444; /* Medium gray */
    border-radius: 6px;
  }
  .table-wrapper::-webkit-scrollbar-thumb:hover {
    background: #666; /* Lighter gray on hover */
  }
</style>