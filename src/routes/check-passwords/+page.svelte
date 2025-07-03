<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import zxcvbn from 'zxcvbn';
  import Chart from 'chart.js/auto';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  import breaches from "../../breaches.json"

  console.log(breaches)

  interface Entry {
    id: number;
    icon: string;
    account: string;
    username: string;
    url: string;
    lastChangeDays: number;
    password: string;
    passwordStrength: 'weak' | 'medium' | 'strong' | null;
    breachStatus: {
      account: { status: string; result: number | null; details?: any[] };
      domain: { status: string; result: number | null; details?: any[] };
    };
  }

  let entries = $state<Entry[]>([]);
  let search = $state('');
  let sortKey = $state<keyof Entry>('account');
  let sortAsc = $state(true);
  let status = $state<'loading' | 'success' | 'empty' | 'error'>('loading');
  let checkingBreaches = $state(false);

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

  let filtered = $derived(
    entries
      .filter(e => e.account.toLowerCase().includes(search.toLowerCase()))
      .sort((a, b) => {
        const aVal = a[sortKey];
        const bVal = b[sortKey];
        const order = sortAsc ? 1 : -1;
        if (typeof aVal === 'string' && typeof bVal === 'string') return aVal.localeCompare(bVal) * order;
        if (typeof aVal === 'number' && typeof bVal === 'number') return (aVal - bVal) * order;
        return 0;
      })
  );

  let stats = $derived({
    total: entries.length,
    breachedAccounts: entries.filter(e => e.breachStatus.account.result && e.breachStatus.account.result > 0).length,
    accountsOnBreachedDomains: entries.filter(e => e.breachStatus.domain.result && e.breachStatus.domain.result > 0).length,
    weakPasswords: entries.filter(e => e.passwordStrength === 'weak').length,
  });

  let ageDistribution = $derived(() => {
    const buckets = { '<1 month': 0, '1-6 months': 0, '6-12 months': 0, '>1 year': 0 };
    entries.forEach(e => {
      if (e.lastChangeDays < 30) buckets['<1 month']++;
      else if (e.lastChangeDays < 180) buckets['1-6 months']++;
      else if (e.lastChangeDays < 365) buckets['6-12 months']++;
      else buckets['>1 year']++;
    });
    return buckets;
  });

  let canvas: HTMLCanvasElement;
  let chart: Chart;

  onMount(async () => {
    try {
      const jsonData: string = await invoke('passwords');
      const raw: any[] = JSON.parse(jsonData);
      const now = Date.now();
      const data: Entry[] = raw.map((p: any, idx: number) => {
        const domain = getDomain(p.url);
        const account = domain ? (websiteAccounts[domain] || domain) : 'Unknown';
        const icon = domain ? `https://${domain}/favicon.ico` : '/icons/default.svg';
        const modified = new Date(p.date_modified).getTime();
        const diffDays = Math.floor((now - modified) / (1000 * 60 * 60 * 24));
        const strength = p.password ? computePasswordStrength(p.password) : null;
        return {
          id: idx + 1,
          icon,
          account,
          username: p.username,
          url: p.url,
          lastChangeDays: diffDays,
          password: p.password,
          passwordStrength: strength,
          breachStatus: {
            account: { status: 'pending', result: null },
            domain: { status: 'pending', result: null },
          },
        };
      });
      entries = data;
      status = data.length ? 'success' : 'empty';

      chart = new Chart(canvas, {
        type: 'bar',
        data: {
          labels: Object.keys(ageDistribution),
          datasets: [{
            label: 'Password Age Distribution',
            data: Object.values(ageDistribution),
            backgroundColor: '#4caf50',
          }],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          scales: { y: { beginAtZero: true } },
          plugins: { legend: { display: false } },
        }
      });
    } catch (e) {
      console.error(e);
      status = 'error';
    }
  });

  $effect(() => {
    if (chart && ageDistribution) {
      chart.data.labels = Object.keys(ageDistribution);
      chart.data.datasets[0].data = Object.values(ageDistribution);
      chart.update();
    }
  });

  onDestroy(() => {
    if (chart) chart.destroy();
  });

  function computePasswordStrength(password: string) {
    const result = zxcvbn(password);
    if (result.score <= 1) return 'weak';
    if (result.score <= 3) return 'medium';
    return 'strong';
  }

  async function checkDomainBreaches(domain: string): Promise<{ count: number; details: { name: string; date: string }[] }> {
    await new Promise(resolve => setTimeout(resolve, 10)); // 10ms delay
    const breach = breaches.find(b => b.Domain === domain);
    const details = breach ? [{ name: breach.Name, date: breach.BreachDate }] : [];
    return { count: details.length, details };
  }

  async function checkAccountBreaches(account: string): Promise<{ count: number; details: any[] }> {
    await new Promise(resolve => setTimeout(resolve, 10)); // 10ms delay
    // Placeholder; can be enhanced with an API like Have I Been Pwned
    return { count: 0, details: [] };
  }

  async function performBreachChecks() {
    checkingBreaches = true;
    for (const entry of entries) {
      // Account check (placeholder)
      entry.breachStatus.account.status = 'checking';
      try {
        const { count, details } = await checkAccountBreaches(entry.username);
        entry.breachStatus.account.result = count;
        entry.breachStatus.account.details = details;
        entry.breachStatus.account.status = 'done';
      } catch {
        entry.breachStatus.account.status = 'error';
      }

      // Domain check
      entry.breachStatus.domain.status = 'checking';
      try {
        const d = getDomain(entry.url);
        const { count, details } = await checkDomainBreaches(d);
        entry.breachStatus.domain.result = count;
        entry.breachStatus.domain.details = details;
        entry.breachStatus.domain.status = 'done';
      } catch {
        entry.breachStatus.domain.status = 'error';
      }
    }
    checkingBreaches = false;
  }

  function toggleSort(key: keyof Entry) {
    if (sortKey === key) {
      sortAsc = !sortAsc;
    } else {
      sortKey = key;
      sortAsc = true;
    }
  }
</script>

<div class="page-wrapper">
  <h1>Password Health Check</h1>
  <div class="stats-dashboard">
    <div class="stat-card"><span>Total Accounts</span><span>{stats.total}</span></div>
    <div class="stat-card"><span>Accounts with Breaches</span><span>{stats.breachedAccounts}</span></div>
    <div class="stat-card"><span>Accounts on Breached Domains</span><span>{stats.accountsOnBreachedDomains}</span></div>
    <div class="stat-card"><span>Weak Passwords</span><span>{stats.weakPasswords}</span></div>
  </div>
  <div class="chart-container">
    <canvas bind:this={canvas}></canvas>
  </div>
  <div class="toolbar">
    <div class="search-wrapper">
      <img class="search-icon" src="/icons/search.svg" alt="" />
      <input type="text" placeholder="Search" bind:value={search} aria-label="Search accounts" />
    </div>
    <button onclick={performBreachChecks} disabled={checkingBreaches}>
      {#if checkingBreaches}Checking...{:else}Check for Breaches{/if}
    </button>
  </div>

  {#if status === 'loading'}
    <div class="status-message">Loading...</div>
  {:else if status === 'empty'}
    <div class="status-message">No passwords found.</div>
  {:else if status === 'error'}
    <div class="status-message error">Failed to load passwords.</div>
  {:else}
    <div class="table-wrapper">
      <table class="entries" role="grid">
        <thead>
          <tr>
            <th onclick={() => toggleSort('account')}>Account</th>
            <th onclick={() => toggleSort('username')}>Username</th>
            <th onclick={() => toggleSort('lastChangeDays')}>Last Password Change</th>
            <th>Security Status</th>
          </tr>
        </thead>
        <tbody>
          {#each filtered as e}
            <tr>
              <td class="account-cell">
                <img src={e.icon} alt={e.account} />
                {e.account}
              </td>
              <td>{e.username}</td>
              <td class="last-change { e.lastChangeDays < 180 ? 'recent' : e.lastChangeDays < 365 ? 'moderate' : 'stale' }">
                {#if e.lastChangeDays < 30}
                  {e.lastChangeDays} days ago
                {:else if e.lastChangeDays < 365}
                  {Math.floor(e.lastChangeDays/30)} months ago
                {:else}
                  {Math.floor(e.lastChangeDays/365)} years ago
                {/if}
              </td>
              <td class="security-status">
                {#if e.breachStatus.account.status === 'checking'}
                  <span class="spinner">⌛</span>
                {:else if e.breachStatus.account.status === 'done'}
                  <span class="status-icon" title={e.breachStatus.account.result ? `${e.breachStatus.account.result} breaches found` : 'No breaches'}>
                    {e.breachStatus.account.result ? '⚠️' : '✅'}
                  </span>
                {/if}

                {#if e.breachStatus.domain.status === 'checking'}
                  <span class="spinner">⌛</span>
                {:else if e.breachStatus.domain.status === 'done'}
                  <span class="status-icon" title={e.breachStatus.domain.result ? `${e.breachStatus.domain.result} domain breaches` : 'No domain breaches'}>
                    {e.breachStatus.domain.result ? '🌐⚠️' : '✅'}
                  </span>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
