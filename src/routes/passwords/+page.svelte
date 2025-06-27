<script lang="ts">
  import { onMount } from 'svelte';
  import { writable, derived } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-shell';
  import { fade, fly, slide } from 'svelte/transition';
  import { flip } from 'svelte/animate';

  import Topbar from '$lib/pages/passwords/topbar.svelte';
    import TooltipContent from '$lib/components/ui/tooltip/tooltip-content.svelte';

  interface PasswordEntry {
    id: number;
    icon: string;
    account: string;
    username: string;
    url: string;
    lastChangeDays: number;
  }

  interface Profile {
    name: string;
    passwords: PasswordEntry[];
  }

  interface Browser {
    name: string;
    profiles: Profile[];
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

  const browserIcons: { [key: string]: string } = {
    'Google Chrome': '/icons/chrome.svg',
    'Microsoft Edge': '/icons/edge.svg',
    'Firefox': '/icons/firefox.svg',
    'Vivaldi': '/icons/vivaldi.svg',
    'Opera GX': '/icons/operagx.svg',
    'Opera': '/icons/opera.svg',
    'Chromium': '/icons/chromium.svg',
    'Brave Browser': '/icons/brave.svg'
  };

  // Stores
  const browserData = writable<Browser[]>([]);
  const selectedBrowserName = writable<string | null>(null);
  const search = writable('');
  const status = writable<'loading' | 'success' | 'empty' | 'error'>('loading');
  const expandedProfiles = writable<Set<string>>(new Set());
  const showDeleteConfirmation = writable<{ profile: Profile | null }>({ profile: null });
  const sortKey = writable<keyof PasswordEntry>('account');
  const sortAsc = writable(true);
  const showSortDropdown = writable(false);

  // Sort options
  const sortOptions = [
    { key: 'account', asc: true, label: 'Account A-Z' },
    { key: 'account', asc: false, label: 'Account Z-A' },
    { key: 'lastChangeDays', asc: false, label: 'Oldest First' },
    { key: 'lastChangeDays', asc: true, label: 'Recent First' }
  ];

  // Derived stores
  const selectedBrowser = derived(
    [browserData, selectedBrowserName],
    ([$browserData, $selectedBrowserName]) =>
      $browserData.find(b => b.name === $selectedBrowserName) || null
  );

  // Auto-expand profiles with search matches
  $: if ($search && $selectedBrowser) {
    const matches = new Set<string>();
    $selectedBrowser.profiles.forEach(profile => {
      if (
        profile.passwords.some(
          p =>
            p.account.toLowerCase().includes($search.toLowerCase()) ||
            p.username.toLowerCase().includes($search.toLowerCase())
        )
      ) {
        matches.add(profile.name);
      }
    });
    expandedProfiles.update(prev => new Set([...prev, ...matches]));
  }

  function toggleProfile(profileName: string) {
    expandedProfiles.update(prev => {
      const next = new Set(prev);
      next.has(profileName) ? next.delete(profileName) : next.add(profileName);
      return next;
    });
  }

  async function deleteProfile(profile: Profile) {
    if (confirm(`Are you sure you want to delete all passwords in profile "${profile.name}"?`)) {
      browserData.update(browsers =>
        browsers.map(browser =>
          browser.name === $selectedBrowserName
            ? { ...browser, profiles: browser.profiles.filter(p => p.name !== profile.name) }
            : browser
        )
      );
    }
  }

  function setSort(key: keyof PasswordEntry, asc: boolean) {
    sortKey.set(key);
    sortAsc.set(asc);
    showSortDropdown.set(false);
  }

  // Open external URL via Tauri shell API
  function openExternal(url: string) {
    open(url).catch(console.error);
  }

  let showPopup = false;
  function togglePopup() {
    showPopup = !showPopup;
  }

  function clickOutside(node: HTMLElement) {
    const handleClick = (event: MouseEvent) => {
      if (!node.contains(event.target as Node)) {
        showPopup = false;
        showSortDropdown.set(false);
      }
    };
    document.addEventListener('click', handleClick, true);
    return {
      destroy() {
        document.removeEventListener('click', handleClick, true);
      }
    };
  }

  onMount(async () => {
    try {
      const jsonData: string = await invoke('passwords');
      const raw = JSON.parse(jsonData);
      let idCounter = 1;
      const now = Date.now();
      const browsers: Browser[] = raw.map((b: any) => ({
        name: b.browser,
        profiles: b.profiles.map((p: any) => ({
          name: p.profile_name,
          passwords: p.passwords.map((pw: any) => {
            const domain = getDomain(pw.url);
            const account = domain ? websiteAccounts[domain] || domain : 'Unknown';
            const icon = domain ? `https://${domain}/favicon.ico` : '/icons/default.svg';
            const modified = new Date(pw.date_modified).getTime();
            const diffDays = Math.floor((now - modified) / (1000 * 60 * 60 * 24));
            return {
              id: idCounter++,
              icon,
              account,
              username: pw.username,
              url: pw.url,
              lastChangeDays: diffDays
            };
          })
        }))
      }));
      browserData.set(browsers);
      if (browsers.length) selectedBrowserName.set(browsers[0].name);
      status.set(browsers.length ? 'success' : 'empty');
    } catch (e) {
      console.error(e);
      status.set('error');
    }
  });

  $: if ($selectedBrowserName && $browserData) {
    const browser = $browserData.find(b => b.name === $selectedBrowserName);
    if (browser?.profiles.length) {
      expandedProfiles.update(prev => new Set([...prev, browser.profiles[0].name]));
    }
  }

  function deleteProfileBackend(browser: string, profile: string) {
    invoke('delete_profile', {
      browserName: browser,
      profileName: profile
    });

  }


</script>

<div class="page-wrapper">
  <Topbar />

  <!-- MAIN CONTENT -->
  <!-- {#if $status === 'loading'}
    <div class="status-message">Loading...</div>
  {:else if $status === 'error'}
    <div class="status-message error">Failed to load passwords.</div>
  {:else if $status === 'empty'}
    <div class="status-message">No browsers found.</div>
  {:else}
    {#if $selectedBrowser && $selectedBrowser.profiles.length}
      <div class="profile-list">
        {#each $selectedBrowser.profiles as profile (profile.name)}
          <div class="profile-item">
            
            <div class="profile-header" on:click={() => toggleProfile(profile.name)}>
              <div class="profile-name">
                {profile.name} ({profile.passwords.length})
              </div>
              <div class="profile-actions">
                <button
                  class="arrow-button"
                  on:click|stopPropagation={() => toggleProfile(profile.name)}
                  aria-label={$expandedProfiles.has(profile.name) ? 'Collapse' : 'Expand'}
                >
                  {#if $expandedProfiles.has(profile.name)}
                    <svg class="arrow" viewBox="0 0 24 24"><path d="M7 14l5-5 5 5z" /></svg>
                  {:else}
                    <svg class="arrow" viewBox="0 0 24 24"><path d="M7 10l5 5 5-5z" /></svg>
                  {/if}
                </button>
                <button
                  class="delete-button"
                  on:click|stopPropagation={() =>
                    showDeleteConfirmation.set({ profile })
                  }
                  aria-label="Delete profile"
                >
                sa moara mama ca nu merge svg asta
                </button>
              </div>
            </div>

            {#if $expandedProfiles.has(profile.name)}
              <div class="password-list" transition:slide>
                {#each profile.passwords
                  .filter(
                    p =>
                      p.account.toLowerCase().includes($search.toLowerCase()) ||
                      p.username.toLowerCase().includes($search.toLowerCase())
                  )
                  .sort((a, b) => {
                    const order = $sortAsc ? 1 : -1;
                    const aVal = a[$sortKey];
                    const bVal = b[$sortKey];
                    if (typeof aVal === 'string')
                      return (aVal as string).localeCompare(bVal as string) * order;
                    return (aVal > bVal ? 1 : -1) * order;
                  }) as password}
                  <div class="password-item">
                    <img
                      src={password.icon}
                      alt={password.account}
                      on:error={e => (e.target.src = '/icons/default.svg')}
                    />
                    <div class="password-details">
                      <div class="account">{password.account}</div>
                      <div class="username">{password.username}</div>
                    </div>
                    <div class="password-age">
                      {#if password.lastChangeDays < 30}
                        {password.lastChangeDays} days ago
                      {:else if password.lastChangeDays < 365}
                        {Math.floor(password.lastChangeDays / 30)} months ago
                      {:else}
                        {Math.floor(password.lastChangeDays / 365)} years ago
                      {/if}
                    </div>
                    
                    <a
                      href={password.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      class="manage-btn"
                      aria-label={`Manage ${password.account}`}
                    >
                      <img src="/icons/arrow.svg" alt="" />
                    </a>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {:else if $selectedBrowser}
      <p class="status-message">No profiles found for this browser.</p>
    {/if}
  {/if} -->

  <!-- DELETE CONFIRMATION MODAL -->
  {#if $showDeleteConfirmation.profile}
    <div class="confirmation-modal">
      <div class="modal-content">
        <h3>Confirm Delete</h3>
        <p>
          Are you sure you want to delete profile "{$showDeleteConfirmation.profile.name}" and
          all its passwords?
        </p>
        <div class="modal-actions">
          <button on:click={() => showDeleteConfirmation.set({ profile: null })}>Cancel</button>
          <button class="confirm" on:click={() => {
            deleteProfileBackend($selectedBrowserName, $showDeleteConfirmation.profile?.name);
            deleteProfile($showDeleteConfirmation.profile);
            showDeleteConfirmation.set({ profile: null });
          }}>Delete</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  /* ================  GLOBAL  ================ */
  @font-face {
    font-family: 'Lexend';
    src: url('/fonts/Lexend.ttf') format('truetype');
    font-weight: 400;
    font-style: normal;
  }

  :global(:root) {
    --panel: #141414;
    --text: #e0e0e0;
    --muted: #777;
    --hover: #272727;
    --border: #444;
  }

  /* ================  LAYOUT  ================ */
  .page-wrapper {
    display: flex;
    flex-direction: column;
    height: 90vh;
    overflow: hidden;
    background: var(--panel);
    font-family: 'Lexend', sans-serif;
  }

  /* ================  TOOLBAR  ================ */
  .toolbar {
    padding: 1rem;
    background: #0d0d0d;
    flex: none;
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  /* Search */
  .search-wrapper {
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
    filter: invert(1);
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
    font-family: 'Lexend', sans-serif;
  }

  input::placeholder {
    color: var(--muted);
  }

  /* ================  BROWSER SELECTOR  ================ */
  .browser-selector {
    position: relative;
  }

  .selected-browser {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: var(--panel);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: transform 0.2s ease;
  }

  .selected-browser:hover {
    transform: scale(1.05);
  }

  .popup {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    background: #333;
    border: 1px solid #555;
    border-radius: 10px;
    padding: 0.5rem;
    z-index: 10;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .popup img {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    padding: 4px;
    background: var(--panel);
    transition: transform 0.2s ease, background 0.2s ease;
    cursor: pointer;
  }

  .popup img:hover {
    transform: scale(1.1);
    background: var(--hover);
  }
  .popup img.selected {
    border: 2px solid var(--text);
  }

  /* ================  PROFILE LIST  ================ */
  .profile-list {
    flex: 1;
    overflow-y: auto;
    padding: 0 1rem;
  }

  .profile-item {
    margin-bottom: 1rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
  }

  .profile-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: var(--hover);
    cursor: pointer;
  }

  .profile-header:hover {
    background: #2a2a2a;
  }
  .profile-name {
    font-weight: 500;
    color: var(--text);
  }
  .profile-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  /* Arrow & delete */
  .arrow-button,
  .delete-button {
    background: transparent;
    border: none;
    padding: 0.5rem;
    cursor: pointer;
    color: var(--text);
    transition: opacity 0.2s ease;
  }

  .delete-button {
    color: #ff4444;
  }
  .delete-button:hover {
    opacity: 0.8;
  }

  .arrow {
    width: 1.5rem;
    height: 1.5rem;
    fill: currentColor;
  }

  /* ================  PASSWORD LIST  ================ */
  .password-list {
    background: #1a1a1a;
    max-height: 400px;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .password-list::-webkit-scrollbar {
    width: 8px;
  }
  .password-list::-webkit-scrollbar-thumb {
    background: #444;
    border-radius: 4px;
  }

  .password-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    border-bottom: 1px solid var(--border);
  }

  .password-item:last-child {
    border-bottom: none;
  }

  .password-item img {
    width: 2rem;
    height: 2rem;
    object-fit: contain;
    border-radius: 4px;
  }

  .password-details {
    flex: 1;
  }
  .account {
    font-weight: 500;
    color: var(--text);
  }
  .username,
  .password-age {
    color: var(--muted);
    font-size: 0.9em;
  }

  /* Manage button */
  .manage-btn {
    background: transparent;
    border: none;
    padding: 0.25rem;
    border-radius: 6px;
    display: inline-flex;
    align-items: center;
    cursor: pointer;
  }

  .manage-btn img {
    width: 1.25rem;
    height: 1.25rem;
    filter: brightness(0) invert(1);
  }

  .manage-btn:hover {
    background: var(--hover);
  }

  /* ================  SORT SELECTOR  ================ */
  .sort-selector {
    position: relative;
  }

  .sort-button {
    background: var(--hover);
    border: none;
    color: var(--text);
    padding: 0.75rem 1.25rem;
    border-radius: 100px;
    display: flex;
    gap: 0.75rem;
    align-items: center;
    font-family: 'Lexend', sans-serif;
    transition: background 0.2s ease, transform 0.2s ease;
  }

  .sort-button:hover {
    background: #2a2a2a;
    transform: translateY(-1px);
  }
  .sort-arrow {
    width: 1rem;
    height: 1rem;
    fill: currentColor;
  }

  .sort-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    left: 0;
    background: #1a1a1a;
    border: 1px solid var(--border);
    border-radius: 20px;
    z-index: 100;
    min-width: 9rem;
    font-family: 'Lexend', sans-serif;
  }

  .sort-option {
    padding: 0.75rem 1rem;
    border-radius: 20px;
    cursor: pointer;
    transition: background 0.2s ease;
    font-size: 0.95rem;
  }

  .sort-option:hover {
    background: var(--hover);
  }
  .sort-option.active {
    background: var(--panel);
  }

  /* ================  MODALS & STATUS  ================ */
  .confirmation-modal {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: var(--panel);
    padding: 2rem;
    border-radius: 8px;
    max-width: 400px;
    width: 90%;
  }

  .modal-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1.5rem;
  }

  .modal-actions button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .modal-actions .confirm {
    background: #ff4444;
    color: #fff;
  }

  .status-message {
    padding: 1rem;
    color: var(--muted);
    text-align: center;
  }

  .status-message.error {
    color: #f44336;
  }
</style>
