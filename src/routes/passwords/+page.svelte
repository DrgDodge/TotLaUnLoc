<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { fade, fly, slide } from 'svelte/transition';
  import { flip } from 'svelte/animate';

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

  const websiteAccounts: Record<string, string> = {
    'google.com': 'Google',
    'facebook.com': 'Facebook',
    'dropbox.com': 'Dropbox',
    'twitter.com': 'Twitter',
    'linkedin.com': 'LinkedIn',
    'github.com': 'GitHub',
    'microsoft.com': 'Microsoft',
    'amazon.com': 'Amazon',
  };

  const browserIcons: Record<string, string> = {
    'Google Chrome': '/icons/chrome.svg',
    'Microsoft Edge': '/icons/edge.svg',
    'Firefox': '/icons/firefox.svg',
    'Vivaldi': '/icons/vivaldi.svg',
    'Opera GX': '/icons/operagx.svg',
    'Opera': '/icons/opera.svg',
    'Chromium': '/icons/chromium.svg',
    'Brave Browser': '/icons/brave.svg'
  };

  // State variables
  let browserData: Browser[] = [];
  let selectedBrowserName: string | null = null;
  let search = '';
  let status: 'loading' | 'success' | 'empty' | 'error' = 'loading';
  let expandedProfiles = new Set<string>();
  let showDeleteConfirmation: { profile: Profile | null } = { profile: null };
  let sortKey: keyof PasswordEntry = 'account';
  let sortAsc = true;
  let showSortDropdown = false;
  let showBrowserPopup = false;

  // Derived state
  $: selectedBrowser = browserData.find(b => b.name === selectedBrowserName) || null;
  
  $: if (search && selectedBrowser) {
    const matches = new Set<string>();
    selectedBrowser.profiles.forEach(profile => {
      if (profile.passwords.some(p => 
        p.account.toLowerCase().includes(search.toLowerCase()) ||
        p.username.toLowerCase().includes(search.toLowerCase())
      )) {
        matches.add(profile.name);
      }
    });
    expandedProfiles = new Set([...expandedProfiles, ...matches]);
  }

  function getDomain(url: string): string {
    try {
      const { hostname } = new URL(url);
      return hostname.replace(/^www\./, '').split('.').slice(-2).join('.');
    } catch {
      return '';
    }
  }

  function toggleProfile(profileName: string) {
    expandedProfiles = new Set(expandedProfiles);
    expandedProfiles.has(profileName) 
      ? expandedProfiles.delete(profileName) 
      : expandedProfiles.add(profileName);
  }

  function triggerDeleteConfirmation(profile: Profile) {
    showDeleteConfirmation = { profile };
  }

  async function confirmDeleteProfile() {
    if (showDeleteConfirmation.profile) {
      try {
        await invoke('delete_profile', { profileName: showDeleteConfirmation.profile.name });
        // Refresh data after deletion
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
              const account = domain ? (websiteAccounts[domain] || domain) : 'Unknown';
              const icon = domain ? `https://${domain}/favicon.ico` : '/icons/default.svg';
              const modified = new Date(pw.date_modified).getTime();
              const diffDays = Math.floor((now - modified) / (1000 * 60 * 60 * 24));
              
              return {
                id: idCounter++,
                icon,
                account,
                username: pw.username,
                url: pw.url,
                lastChangeDays: diffDays,
              };
            }),
          })),
        }));
        
        browserData = browsers;
        if (browsers.length > 0) {
          selectedBrowserName = browsers[0].name;
          expandedProfiles = new Set();
        }
        status = browsers.length ? 'success' : 'empty';

        showDeleteConfirmation = { profile: null }; // Close modal
      } catch (error) {
        console.error('Error deleting profile:', error);
        // Optionally, show an error message to the user
      }
    }
  }

  

  function setSort(key: keyof PasswordEntry, asc: boolean) {
    sortKey = key;
    sortAsc = asc;
    showSortDropdown = false;
  }

  function clickOutside(node: HTMLElement) {
    const handleClick = (event: MouseEvent) => {
      if (!node.contains(event.target as Node)) {
        showBrowserPopup = false;
        showSortDropdown = false;
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
            const account = domain ? (websiteAccounts[domain] || domain) : 'Unknown';
            const icon = domain ? `https://${domain}/favicon.ico` : '/icons/default.svg';
            const modified = new Date(pw.date_modified).getTime();
            const diffDays = Math.floor((now - modified) / (1000 * 60 * 60 * 24));
            
            return {
              id: idCounter++,
              icon,
              account,
              username: pw.username,
              url: pw.url,
              lastChangeDays: diffDays,
            };
          }),
        })),
      }));
      
      browserData = browsers;
      if (browsers.length > 0) {
        selectedBrowserName = browsers[0].name;
        expandedProfiles = new Set();
      }
      status = browsers.length ? 'success' : 'empty';
    } catch (e) {
      console.error(e);
      status = 'error';
    }
  });

  // Sort options
  const sortOptions: { key: keyof PasswordEntry; asc: boolean; label: string }[] = [
    { key: 'account', asc: true, label: 'Account A-Z' },
    { key: 'account', asc: false, label: 'Account Z-A' },
    { key: 'lastChangeDays', asc: false, label: 'Oldest First' },
    { key: 'lastChangeDays', asc: true, label: 'Recent First' }
  ];
</script>

<div class="page-wrapper">
  <div class="toolbar">
    <div class="search-wrapper">
      <img class="search-icon" src="/icons/search.svg" alt="Search" />
      <input 
        type="text" 
        placeholder="Search" 
        bind:value={search}
        aria-label="Search accounts" 
      />
    </div>

    <div class="sort-selector">
      <button 
        class="sort-button" 
        on:click={() => showSortDropdown = !showSortDropdown}
      >
        {sortOptions.find(o => o.key === sortKey && o.asc === sortAsc)?.label}
        <svg class="sort-arrow" viewBox="0 0 24 24">
          <path d="M7 10l5 5 5-5z" />
        </svg>
      </button>
      
      {#if showSortDropdown}
        <div class="sort-dropdown" use:clickOutside>
          {#each sortOptions as option}
            <button
              class="sort-option {option.key === sortKey && option.asc === sortAsc ? 'active' : ''}"
              on:click={() => setSort(option.key, option.asc)}
            >
              {option.label}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <div class="browser-selector">
      <button class="selected-browser" on:click={() => showBrowserPopup = !showBrowserPopup}>
        {#if selectedBrowserName && browserIcons[selectedBrowserName]}
          <img 
            src={browserIcons[selectedBrowserName]} 
            alt={selectedBrowserName} 
          />
        {:else}
          <div class="placeholder"></div>
        {/if}
      </button>
      
      {#if showBrowserPopup}
        <div class="popup" use:clickOutside transition:fade>
          {#each browserData as browser, i (browser.name)}
            <button
              class="browser-icon-button"
              on:click={() => {
                selectedBrowserName = browser.name;
                showBrowserPopup = false;
              }}
              in:fly={{ y: -20, duration: 250, delay: i * 50 }}
              out:fly={{ y: 20, duration: 250 }}
              animate:flip
            >
              <img
                src={browserIcons[browser.name] || '/icons/default.svg'}
                alt={browser.name}
                class:selected={browser.name === selectedBrowserName}
              />
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  {#if status === 'loading'}
    <div class="status-message">Loading...</div>
  {:else if status === 'error'}
    <div class="status-message error">Failed to load passwords.</div>
  {:else if status === 'empty'}
    <div class="status-message">No browsers found.</div>
  {:else if selectedBrowser && selectedBrowser.profiles.length > 0}
    <div class="profile-list">
      {#each selectedBrowser.profiles as profile (profile.name)}
        <div class="profile-item">
          <div class="profile-header" role="button" tabindex="0" on:click={() => toggleProfile(profile.name)} on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') toggleProfile(profile.name); }}>
            <div class="profile-name">
              {profile.name} ({profile.passwords.length})
            </div>
            <div class="profile-actions">
              <button
                class="arrow-button"
                on:click|stopPropagation={() => toggleProfile(profile.name)}
                aria-label={expandedProfiles.has(profile.name) ? 'Collapse' : 'Expand'}
              >
                {#if expandedProfiles.has(profile.name)}
                  <svg class="arrow" viewBox="0 0 24 24"><path d="M7 14l5-5 5 5z"/></svg>
                {:else}
                  <svg class="arrow" viewBox="0 0 24 24"><path d="M7 10l5 5 5-5z"/></svg>
                {/if}
              </button>
              <button
                class="delete-button"
                on:click|stopPropagation={() => triggerDeleteConfirmation(profile)}
                aria-label="Delete profile"
              >
                <img alt="trash" src="icons/trash.svg">
              </button>
            </div>
          </div>
          
          {#if expandedProfiles.has(profile.name)}
            <div class="password-list" transition:slide>
              {#each profile.passwords
                .filter(p => 
                  p.account.toLowerCase().includes(search.toLowerCase()) ||
                  p.username.toLowerCase().includes(search.toLowerCase())
                )
                .sort((a, b) => {
                  const aVal = a[sortKey];
                  const bVal = b[sortKey];
                  const order = sortAsc ? 1 : -1;
                  
                  // Handle different types safely
                  if (sortKey === 'account' || sortKey === 'username') {
                    return (aVal as string).localeCompare(bVal as string) * order;
                  } else if (sortKey === 'lastChangeDays') {
                    return ((aVal as number) - (bVal as number)) * order;
                  }
                  return 0;
                }) as password}
                <div class="password-item">
                  <img 
                    src={password.icon} 
                    alt={password.account}  
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
                    aria-label="Manage {password.account}"
                  >
                    <img src="/icons/arrow.svg" alt="Go" />
                  </a>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {:else if selectedBrowser}
    <p class="status-message">No profiles found for this browser.</p>
  {/if}

  {#if showDeleteConfirmation.profile}
    <div class="confirmation-modal">
      <div class="modal-content">
        <h3>Confirm Delete</h3>
        <p>Are you sure you want to delete profile "{showDeleteConfirmation.profile.name}" and all its passwords?</p>
        <div class="modal-actions">
          <button on:click={() => showDeleteConfirmation = { profile: null }}>Cancel</button>
          <button class="confirm" on:click={confirmDeleteProfile}>Delete</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  @font-face {
    font-family: 'Lexend';
    src: url('/fonts/Lexend.woff2') format('woff2');
    font-weight: 400;
    font-style: normal;
  }

  :root {
    --panel: #141414;
    --text: #e0e0e0;
    --muted: #777;
    --hover: #272727;
    --border: #444;
    --error: #f44336;
  }

  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-family: 'Lexend', sans-serif;
  }


  .page-wrapper {
    display: flex;
    flex-direction: column;
    position: absolute;
    inset: 0;
    background: var(--panel);
    color: var(--text);
    overflow: hidden; /* Prevent main page scroll */
  }

  .toolbar {
    padding: 1rem;
    background: #0d0d0d;
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

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
  }

  input::placeholder {
    color: var(--muted);
  }

  .sort-selector {
    position: relative;
  }

  .sort-button {
    background: var(--hover);
    border: none;
    color: var(--text);
    padding: 0.75rem 1.25rem;
    border-radius: 100px;
    cursor: pointer;
    display: flex;
    gap: 0.75rem;
    align-items: center;
    font-weight: 400;
    transition: all 0.2s ease;
  }

  .sort-button:hover {
    background: #2a2a2a;
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
    overflow: hidden;
  }

  .sort-option {
    padding: 0.75rem 1rem;
    cursor: pointer;
    transition: background 0.2s ease;
    font-size: 0.95rem;
    border: none;
    background: none;
    color: inherit;
    text-align: left;
    width: 100%;
  }

  .sort-option:hover {
    background: var(--hover);
  }

  .sort-option.active {
    background: var(--panel);
  }

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
    border: none; /* Ensure no default border */
    outline: none; /* Remove default focus outline */
  }

  .selected-browser:focus-visible {
    outline: 2px solid var(--text); /* Add a custom focus indicator if desired */
    outline-offset: 2px;
  }

  .selected-browser:hover {
    transform: scale(1.05);
  }

  .selected-browser img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .popup {
    position: absolute;
    top: calc(100% + 0.5rem);
    left: 50%; /* Center horizontally */
    transform: translateX(-50%); /* Adjust for element's own width */
    background: #333;
    border: 1px solid #555;
    border-radius: 10px;
    padding: 0.5rem;
    z-index: 10;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .popup button {
    width: 40px;
    height: 40px;
    cursor: pointer;
    border-radius: 50%;
    transition: transform 0.2s ease, background 0.2s ease;
    padding: 4px;
    background: var(--panel);
    border: none;
  }

  .popup button:hover {
    transform: scale(1.1);
    background: var(--hover);
  }

  .popup button img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .popup button img.selected {
    border: none;
  }

  .profile-list {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    min-height: 0; /* Allow flex item to shrink */
    /* Hide scrollbar for Chrome, Safari and Opera */
    &::-webkit-scrollbar {
      display: none;
    }
    /* Hide scrollbar for IE, Edge and Firefox */
    -ms-overflow-style: none;  /* IE and Edge */
    scrollbar-width: none;  /* Firefox */
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

  .profile-name {
    font-weight: 500;
  }

  .profile-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .arrow-button, .delete-button {
    background: none;
    border: none;
    padding: 0.5rem;
    cursor: pointer;
    color: var(--text);
    display: flex;
  }

  .arrow {
    width: 1.5rem;
    height: 1.5rem;
    fill: currentColor;
  }

  .delete-button {
    color: #ff4444;
    filter: invert();
  }

  .delete-button:hover {
    opacity: 0.8;
  }

  .password-list {
    background: #1a1a1a;
    padding: 0.5rem;
  }

  .password-item {
    display: flex;
    align-items: center;
    padding: 1rem;
    gap: 1rem;
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
  }

  .username {
    color: var(--muted);
    font-size: 0.9em;
  }

  .password-age {
    color: var(--muted);
    font-size: 0.9em;
    min-width: 100px;
    text-align: right;
  }

  .manage-btn {
    background: transparent;
    padding: 0.25rem;
    border-radius: 6px;
    display: inline-flex;
    align-items: center;
    border: none;
    cursor: pointer;
    text-decoration: none;
  }

  .manage-btn img {
    width: 1.25rem;
    height: 1.25rem;
    filter: invert(1);
  }

  .manage-btn:hover {
    background: var(--hover);
  }

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
    color: white;
  }

  .status-message {
    padding: 2rem;
    text-align: center;
    color: var(--muted);
  }

  .error {
    color: var(--error) !important;
  }
</style>