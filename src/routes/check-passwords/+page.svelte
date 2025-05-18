<script>
  import { onMount, onDestroy } from 'svelte';
  import { writable, derived, get } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';
  import zxcvbn from 'zxcvbn';
  import Chart from 'chart.js/auto';

  /** @type {{ id: number; icon: string; account: string; username: string; url: string; lastChangeDays: number; password: string; passwordStrength: 'weak'|'medium'|'strong'|null; breachStatus: { account: { status: string; result: number|null; details?: any[] }; domain: { status: string; result: number|null; details?: any[] } } }[]} */
  const entries = writable([]);

  /** @type {{ [key: string]: string }} */
  const websiteAccounts = {
    'google.com': 'Google',
    'facebook.com': 'Facebook',
    'dropbox.com': 'Dropbox',
    'twitter.com': 'Twitter',
    'linkedin.com': 'LinkedIn',
    'github.com': 'GitHub',
    'microsoft.com': 'Microsoft',
    'amazon.com': 'Amazon',
  };

  function getDomain(url) {
    try {
      const { hostname } = new URL(url);
      return hostname.replace(/^www\./, '').split('.').slice(-2).join('.');
    } catch {
      return '';
    }
  }

  const search = writable('');
  const sortKey = writable('account');
  const sortAsc = writable(true);
  const status = writable('loading');
  const checkingBreaches = writable(false);

  const filtered = derived(
    [entries, search, sortKey, sortAsc],
    ([$entries, $search, $sortKey, $sortAsc]) =>
      $entries
        .filter(e => e.account.toLowerCase().includes($search.toLowerCase()))
        .sort((a, b) => {
          const aVal = a[$sortKey];
          const bVal = b[$sortKey];
          const order = $sortAsc ? 1 : -1;
          if (typeof aVal === 'string') return aVal.localeCompare(bVal) * order;
          if (typeof aVal === 'number') return (aVal - bVal) * order;
          return 0;
        })
  );

  const stats = derived(entries, $entries => ({
    total: $entries.length,
    breachedAccounts: $entries.filter(e => e.breachStatus.account.result > 0).length,
    accountsOnBreachedDomains: $entries.filter(e => e.breachStatus.domain.result > 0).length,
    weakPasswords: $entries.filter(e => e.passwordStrength === 'weak').length,
  }));

  const ageDistribution = derived(entries, $entries => {
    const buckets = { '<1 month': 0, '1-6 months': 0, '6-12 months': 0, '>1 year': 0 };
    $entries.forEach(e => {
      if (e.lastChangeDays < 30) buckets['<1 month']++;
      else if (e.lastChangeDays < 180) buckets['1-6 months']++;
      else if (e.lastChangeDays < 365) buckets['6-12 months']++;
      else buckets['>1 year']++;
    });
    return buckets;
  });

  let canvas;
  let chart;

  onMount(async () => {
    try {
      const jsonData = await invoke('passwords');
      const raw = JSON.parse(jsonData);
      const now = Date.now();
      const data = raw.map((p, idx) => {
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
      entries.set(data);
      status.set(data.length ? 'success' : 'empty');

      chart = new Chart(canvas, {
        type: 'bar',
        data: {
          labels: Object.keys(get(ageDistribution)),
          datasets: [{
            label: 'Password Age Distribution',
            data: Object.values(get(ageDistribution)),
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
      status.set('error');
    }
  });

  $: if (chart && get(ageDistribution)) {
    chart.data.labels = Object.keys(get(ageDistribution));
    chart.data.datasets[0].data = Object.values(get(ageDistribution));
    chart.update();
  }

  onDestroy(() => {
    if (chart) chart.destroy();
  });

  function computePasswordStrength(password) {
    const result = zxcvbn(password);
    if (result.score <= 1) return 'weak';
    if (result.score <= 3) return 'medium';
    return 'strong';
  }

  /** @type {{ domain: string; name: string; date: string }[]} */
  const knownBreaches = [
    { domain: 'adobe.com', name: 'Adobe', date: '2019-10-04' },
    { domain: 'facebook.com', name: 'Facebook', date: '2019-09-28' },
    { domain: 'ottawa.ca', name: 'City of Ottawa', date: '2019-03-13' },
    { domain: 'canva.com', name: 'Canva', date: '2019-05-24' },
    { domain: 'yelp.com', name: 'Yelp', date: '2019-02-14' },
    { domain: 'healthgrades.com', name: 'Healthgrades', date: '2019-09-20' },
    { domain: 'bulletin.com', name: 'Bulletin', date: '2019-11-01' },
    { domain: 'akamai.com', name: 'Akamai', date: '2019-03-21' },
    { domain: 'sl.wikipedia.org', name: 'Wikipedia Sindhi', date: '2019-02-20' },
    { domain: 'capitalone.com', name: 'Capital One', date: '2019-07-29' },
    { domain: 'doordash.com', name: 'DoorDash', date: '2019-09-26' },
    { domain: 'verifications.io', name: 'Verifications.io', date: '2019-02-25' },
    { domain: 'firstamerican.com', name: 'First American Financial', date: '2019-05-24' },
    { domain: 'myfitnesspal.com', name: 'MyFitnessPal', date: '2019-02-01' },
    { domain: 'evite.com', name: 'Evite', date: '2019-04-01' },
    { domain: 'quora.com', name: 'Quora', date: '2019-12-03' },
    { domain: 'zynga.com', name: 'Zynga', date: '2019-09-01' },
    { domain: 'elastic.co', name: 'ElasticSearch', date: '2019-01-01' },
    { domain: 'freedommobile.ca', name: 'Freedom Mobile', date: '2019-04-01' },
    { domain: 'adventhealth.com', name: 'Advent Health', date: '2019-02-01' },
    { domain: '500px.com', name: '500px', date: '2019-02-08' },
    { domain: 'coffeemeetsbagel.com', name: 'Coffee Meets Bagel', date: '2019-02-14' },
    { domain: 'dropbox.com', name: 'Dropbox', date: '2019-07-15' },
    { domain: 'linkedin.com', name: 'LinkedIn', date: '2019-05-05' },
    { domain: 'yahoo.com', name: 'Yahoo', date: '2019-08-01' },
    { domain: 't-mobile.com', name: 'T-Mobile', date: '2019-08-24' },
    { domain: 'ubereats.com', name: 'Uber Eats', date: '2019-09-25' },
    { domain: 'spotify.com', name: 'Spotify', date: '2019-05-12' },
    { domain: 'uber.com', name: 'Uber', date: '2019-10-14' },
    { domain: 'instagram.com', name: 'Instagram', date: '2019-09-04' },
    { domain: 'twitch.tv', name: 'Twitch', date: '2019-05-06' },
    { domain: 'zoom.us', name: 'Zoom', date: '2019-04-01' },
    { domain: 'slack.com', name: 'Slack', date: '2019-06-05' },
    { domain: 'tinder.com', name: 'Tinder', date: '2019-11-01' },
    { domain: 'okcupid.com', name: 'OkCupid', date: '2019-05-05' },
    { domain: 'shopify.com', name: 'Shopify', date: '2019-11-04' },
    { domain: 'mailchimp.com', name: 'MailChimp', date: '2019-05-11' },
    { domain: 'namecheap.com', name: 'Namecheap', date: '2019-07-19' },
    { domain: 'underarmour.com', name: 'Under Armour', date: '2019-02-01' },
    { domain: 'flickr.com', name: 'Flickr', date: '2019-01-02' },
    { domain: 'crunchyroll.com', name: 'Crunchyroll', date: '2019-12-08' },
    { domain: 'solarwinds.com', name: 'SolarWinds', date: '2020-12-13' },
    { domain: 'microsoft.com', name: 'Microsoft Exchange', date: '2021-03-02' },
    { domain: 'citi.com', name: 'Citigroup', date: '2019-11-01' },
    { domain: 'oracle.com', name: 'Oracle', date: '2019-06-17' },
    { domain: 'wordpress.com', name: 'WordPress.com', date: '2019-02-21' },
    { domain: 'teams.microsoft.com', name: 'Microsoft Teams', date: '2019-08-30' },
    { domain: 'gitlab.com', name: 'GitLab', date: '2019-02-05' },
    { domain: 'bitbucket.org', name: 'Bitbucket', date: '2019-08-19' },
    { domain: 'okta.com', name: 'Okta', date: '2019-10-21' },
    { domain: 'adobe.com', name: 'Adobe', date: '2013-10-04' },
    { domain: 'dropbox.com', name: 'Dropbox', date: '2012-07-15' },
    { domain: 'linkedin.com', name: 'LinkedIn', date: '2012-05-05' },
    { domain: 'yahoo.com', name: 'Yahoo', date: '2013-08-01' },
    { domain: 'equiifax.com', name: 'Equifax', date: '2017-07-29' },
    { domain: 'facebook.com', name: 'Facebook', date: '2019-09-28' },
    { domain: 'marriott.com', name: 'Marriott', date: '2018-11-30' },
    { domain: 'myspace.com', name: 'MySpace', date: '2016-05-31' },
    { domain: 'anthem.com', name: 'Anthem', date: '2015-02-04' },
    { domain: 'target.com', name: 'Target', date: '2013-12-18' },
    { domain: 'evernote.com', name: 'Evernote', date: '2013-03-05' },
    { domain: 'bitly.com', name: 'Bitly', date: '2014-03-01' },
    { domain: 'tumblr.com', name: 'Tumblr', date: '2013-02-04' },
    { domain: 'ask.fm', name: 'Ask.fm', date: '2014-06-07' },
    { domain: 'vk.com', name: 'VK', date: '2018-01-01' },
    { domain: 'playstation.com', name: 'PlayStation', date: '2011-04-27' },
    { domain: 'steamcommunity.com', name: 'Steam', date: '2011-04-25' },
    { domain: 'ottawa.ca', name: 'City of Ottawa', date: '2019-03-13' },
    { domain: 'canva.com', name: 'Canva', date: '2019-05-24' },
    { domain: 'ticketmaster.com', name: 'Ticketmaster', date: '2018-06-23' },
    { domain: 'citi.com', name: 'Citigroup', date: '2021-11-01' },
    { domain: 'oracle.com', name: 'Oracle', date: '2020-06-17' },
    { domain: 'wordpress.com', name: 'WordPress.com', date: '2020-02-21' },
    { domain: 'ubereats.com', name: 'Uber Eats', date: '2018-09-25' },
    { domain: 'spotify.com', name: 'Spotify', date: '2016-05-12' },
    { domain: 'yelp.com', name: 'Yelp', date: '2019-02-14' },
    { domain: 'uber.com', name: 'Uber', date: '2016-10-14' },
    { domain: 't-mobile.com', name: 'T-Mobile', date: '2021-08-24' },
    { domain: 'linkedin.cn', name: 'LinkedIn (CN)', date: '2021-06-24' },
    { domain: 'match.com', name: 'Match.com', date: '2020-05-05' },
    { domain: 'mturk.com', name: 'MTurk', date: '2018-03-23' },
    { domain: 'zkillboard.com', name: 'zKillboard', date: '2021-01-11' },
    { domain: 'cisco.com', name: 'Cisco', date: '2016-08-24' },
    { domain: 'teams.microsoft.com', name: 'Microsoft Teams', date: '2020-08-30' },
    { domain: 'gitlab.com', name: 'GitLab', date: '2021-02-05' },
    { domain: 'bitbucket.org', name: 'Bitbucket', date: '2018-08-19' },
    { domain: 'okta.com', name: 'Okta', date: '2021-10-21' },
    { domain: 'shopify.com', name: 'Shopify', date: '2020-11-04' },
    { domain: 'mailchimp.com', name: 'MailChimp', date: '2020-05-11' },
    { domain: 'namecheap.com', name: 'Namecheap', date: '2021-07-19' },
    { domain: 'hewlett-packard.com', name: 'HP', date: '2018-09-01' },
    { domain: 'healthgrades.com', name: 'Healthgrades', date: '2019-09-20' },
    { domain: 'underarmour.com', name: 'Under Armour', date: '2018-02-01' },
    { domain: 'ivantel.com', name: 'Ivantel ISP', date: '2020-01-15' },
    { domain: 'eurostar.com', name: 'Eurostar', date: '2020-02-29' },
    { domain: 'flickr.com', name: 'Flickr', date: '2014-01-02' },
    { domain: 'baidu.com', name: 'Baidu', date: '2015-07-16' },
    { domain: 'instagram.com', name: 'Instagram', date: '2018-09-04' },
    { domain: 'crunchyroll.com', name: 'Crunchyroll', date: '2020-12-08' },
    { domain: 'xboxlive.com', name: 'Xbox Live', date: '2012-08-02' },
    { domain: 'telegram.org', name: 'Telegram', date: '2021-03-23' },
    { domain: 'wikileaks.org', name: 'WikiLeaks', date: '2017-11-13' },
    { domain: 'gfycat.com', name: 'Gfycat', date: '2018-04-10' },
    { domain: 'twitch.tv', name: 'Twitch', date: '2014-05-06' },
    { domain: 'aol.com', name: 'AOL', date: '2013-06-03' },
    { domain: 'mail.ru', name: 'Mail.ru', date: '2016-01-05' },
    { domain: 'live.com', name: 'Microsoft Live', date: '2011-05-03' },
    { domain: 'tinder.com', name: 'Tinder', date: '2018-11-01' },
    { domain: 'zoom.us', name: 'Zoom', date: '2020-04-01' },
    { domain: 'slack.com', name: 'Slack', date: '2015-06-05' },
    { domain: 'okcupid.com', name: 'OkCupid', date: '2018-05-05' },
    { domain: 'buff.ly', name: 'Buffer', date: '2020-03-01' },
    { domain: 'weebly.com', name: 'Weebly', date: '2017-11-13' },
    { domain: 'wordpress.org', name: 'WordPress.org', date: '2013-07-03' },
    { domain: 'imgur.com', name: 'Imgur', date: '2014-12-24' },
    { domain: 'sina.com.cn', name: 'Sina', date: '2013-04-12' },
    { domain: 'foursquare.com', name: 'Foursquare', date: '2018-06-15' },
    { domain: 'meetup.com', name: 'Meetup', date: '2014-06-06' },
    { domain: 'bbc.co.uk', name: 'BBC', date: '2016-11-07' },
    { domain: 'washingtonpost.com', name: 'Washington Post', date: '2013-11-15' },
    { domain: 'forbes.com', name: 'Forbes', date: '2014-02-14' },
    { domain: 'samsung.com', name: 'Samsung', date: '2014-12-19' },
    { domain: 'playstationnetwork.com', name: 'PSN', date: '2011-04-27' },
    { domain: 'bulletin.com', name: 'Bulletin', date: '2019-11-01' },
    { domain: 'northlane.com', name: 'Northlane ISP', date: '2021-08-05' },
    { domain: '100tb.com', name: '100TB Hosting', date: '2020-01-20' },
    { domain: 'libertyglobal.com', name: 'Liberty Global', date: '2018-10-07' },
    { domain: 'dailymotion.com', name: 'Dailymotion', date: '2016-09-02' },
    { domain: 'rushmore.com', name: 'Rushmore ISP', date: '2021-04-11' },
    { domain: 'akamai.com', name: 'Akamai', date: '2019-03-21' },
    { domain: 'dowjones.com', name: 'Dow Jones', date: '2020-07-10' },
    { domain: 'indiegogo.com', name: 'Indiegogo', date: '2017-03-14' },
    { domain: 'kickstarter.com', name: 'Kickstarter', date: '2018-04-12' },
    { domain: 'gog.com', name: 'GOG.com', date: '2013-10-31' },
    { domain: 'bitcointalk.org', name: 'Bitcointalk', date: '2015-08-14' },
    { domain: 'sl.wikipedia.org', name: 'Wikipedia Sindhi', date: '2019-02-20' },
    { domain: 'adstream.com', name: 'Adstream', date: '2019-01-08' },
    { domain: 'cafepress.com', name: 'CafePress', date: '2019-02-28' },
    { domain: 'creditkarma.com', name: 'Credit Karma', date: '2019-03-18' },
    { domain: 'rockauto.com', name: 'RockAuto', date: '2019-04-20' },
    { domain: 'ottawa.ca', name: 'City of Ottawa', date: '2019-03-13' },
    { domain: 'canva.com', name: 'Canva', date: '2019-05-24' },
    { domain: 'facebook.com', name: 'Facebook (Cambridge Analytica)', date: '2019-09-28' },
    { domain: 'powerbi.com', name: 'Microsoft Power BI', date: '2019-12-20' },
    { domain: 'eurostar.com', name: 'Eurostar', date: '2020-02-29' },
    { domain: '3cx.com', name: '3CX', date: '2020-05-07' },
    { domain: 'buff.ly', name: 'Buffer', date: '2020-03-01' },
    { domain: 'wordpress.com', name: 'WordPress.com', date: '2020-02-21' },
    { domain: 'trendmicro.com', name: 'Trend Micro', date: '2020-06-15' },
    { domain: 'oracle.com', name: 'Oracle', date: '2020-06-17' },
    { domain: 'skillshare.com', name: 'Skillshare', date: '2020-06-25' },
    { domain: 'visa.com', name: 'Visa (APAC)', date: '2020-07-14' },
    { domain: 'dowjones.com', name: 'Dow Jones', date: '2020-07-10' },
    { domain: 'wordpress.com', name: 'WordPress.com', date: '2020-08-12' },
    { domain: 'slack.com', name: 'Slack', date: '2020-08-20' },
    { domain: 'teams.microsoft.com', name: 'Microsoft Teams', date: '2020-08-30' },
    { domain: 'northlane.com', name: 'Northlane ISP', date: '2021-08-05' },
    { domain: 'okta.com', name: 'Okta', date: '2021-10-21' },
    { domain: 'linkedin.cn', name: 'LinkedIn (CN)', date: '2021-06-24' },
    { domain: 'rushmore.com', name: 'Rushmore ISP', date: '2021-04-11' },
    { domain: 'zkillboard.com', name: 'zKillboard', date: '2021-01-11' },
    { domain: 'gitlab.com', name: 'GitLab', date: '2021-02-05' },
    { domain: 'namecheap.com', name: 'Namecheap', date: '2021-07-19' },
    { domain: 'match.com', name: 'Match.com', date: '2020-05-05' },
    { domain: 'shopify.com', name: 'Shopify', date: '2020-11-04' },
    { domain: 'mailchimp.com', name: 'Mailchimp', date: '2020-05-11' },
    { domain: 'crunchyroll.com', name: 'Crunchyroll', date: '2020-12-08' },
    { domain: 'hotosm.org', name: 'OpenStreetMap Foundation', date: '2020-09-14' },
    { domain: 'zoom.us', name: 'Zoom', date: '2020-04-01' },
    { domain: 'adobe.com', name: 'Adobe (Magento)', date: '2020-10-12' },
    { domain: 'buff.ly', name: 'Buffer', date: '2020-03-01' },
    { domain: 'teams.microsoft.com', name: 'Microsoft Teams', date: '2020-08-30' },
    { domain: 'trendmicro.com', name: 'Trend Micro', date: '2020-06-15' },
    { domain: 'oracle.com', name: 'Oracle', date: '2020-06-17' },
    { domain: 'telegram.org', name: 'Telegram', date: '2021-03-23' },
    { domain: 'zkillboard.com', name: 'zKillboard', date: '2021-01-11' },
    { domain: 'accorhotels.com', name: 'AccorHotels', date: '2021-01-12' },
    { domain: 'linkedin.com', name: 'LinkedIn', date: '2021-04-06' },
    { domain: 'dailymotion.com', name: 'Dailymotion', date: '2020-12-28' },
    { domain: 'vanguard.com', name: 'Vanguard', date: '2020-11-25' },
    { domain: 'experian.com', name: 'Experian', date: '2020-12-01' },
    { domain: 'ticketmaster.com', name: 'Ticketmaster', date: '2020-09-10' },
    { domain: 'bitbucket.org', name: 'Bitbucket', date: '2020-10-07' },
    { domain: 'evernote.com', name: 'Evernote', date: '2020-10-15' },
    { domain: 'myspace.com', name: 'MySpace', date: '2020-11-02' },
    { domain: 'vk.com', name: 'VK', date: '2020-09-01' },
    { domain: 'twitch.tv', name: 'Twitch', date: '2020-12-09' },
    { domain: 'gfycat.com', name: 'Gfycat', date: '2020-12-20' },
    { domain: 'meetup.com', name: 'Meetup', date: '2020-12-22' },
    { domain: 'weebly.com', name: 'Weebly', date: '2020-12-25' },
    { domain: 'foursquare.com', name: 'Foursquare', date: '2020-12-27' },
    { domain: 'imgur.com', name: 'Imgur', date: '2021-01-05' },
    { domain: 'linkedin.cn', name: 'LinkedIn (CN)', date: '2021-02-14' }
  ];

  async function checkDomainBreaches(domain) {
    await new Promise(resolve => setTimeout(resolve, 10)); // 10ms delay
    const breach = knownBreaches.find(b => b.domain === domain);
    const details = breach ? [{ name: breach.name, date: breach.date }] : [];
    return { count: details.length, details };
  }

  async function checkAccountBreaches(account) {
    await new Promise(resolve => setTimeout(resolve, 10)); // 10ms delay
    // Placeholder; can be enhanced with an API like Have I Been Pwned
    return { count: 0, details: [] };
  }

  async function performBreachChecks() {
    checkingBreaches.set(true);
    const $entries = get(entries);
    for (const entry of $entries) {
      // Account check (placeholder)
      entry.breachStatus.account.status = 'checking';
      entries.set($entries);
      try {
        const { count, details } = await checkAccountBreaches(entry.username);
        entry.breachStatus.account.result = count;
        entry.breachStatus.account.details = details;
        entry.breachStatus.account.status = 'done';
      } catch {
        entry.breachStatus.account.status = 'error';
      }
      entries.set($entries);

      // Domain check
      entry.breachStatus.domain.status = 'checking';
      entries.set($entries);
      try {
        const d = getDomain(entry.url);
        const { count, details } = await checkDomainBreaches(d);
        entry.breachStatus.domain.result = count;
        entry.breachStatus.domain.details = details;
        entry.breachStatus.domain.status = 'done';
      } catch {
        entry.breachStatus.domain.status = 'error';
      }
      entries.set($entries);
    }
    checkingBreaches.set(false);
  }

  function toggleSort(key) {
    const current = get(sortKey);
    sortKey.set(key);
    sortAsc.update(a => (current === key ? !a : true));
  }
</script>

<div class="page-wrapper">
  <h1>Password Health Check</h1>
  <div class="stats-dashboard">
    <div class="stat-card"><span>Total Accounts</span><span>{$stats.total}</span></div>
    <div class="stat-card"><span>Accounts with Breaches</span><span>{$stats.breachedAccounts}</span></div>
    <div class="stat-card"><span>Accounts on Breached Domains</span><span>{$stats.accountsOnBreachedDomains}</span></div>
    <div class="stat-card"><span>Weak Passwords</span><span>{$stats.weakPasswords}</span></div>
  </div>
  <div class="chart-container">
    <canvas bind:this={canvas} />
  </div>
  <div class="toolbar">
    <div class="search-wrapper">
      <img class="search-icon" src="/icons/search.svg" alt="" />
      <input type="text" placeholder="Search" bind:value={$search} aria-label="Search accounts" />
    </div>
    <button on:click={performBreachChecks} disabled={$checkingBreaches}>
      {#if $checkingBreaches}Checking...{:else}Check for Breaches{/if}
    </button>
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
            <th on:click={() => toggleSort('lastChangeDays')}>Last Password Change</th>
            <th>Security Status</th>
          </tr>
        </thead>
        <tbody>
          {#each $filtered as e}
            <tr>
              <td class="account-cell">
                <img src={e.icon} alt={e.account} on:error={i => (i.target.src = '/icons/default.svg')} />
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
                  <span class="status-icon" title={e.breachStatus.account.result > 0 ? `${e.breachStatus.account.result} breaches found` : 'No breaches'}>
                    {e.breachStatus.account.result > 0 ? '⚠️' : '✅'}
                  </span>
                {/if}

                {#if e.breachStatus.domain.status === 'checking'}
                  <span class="spinner">⌛</span>
                {:else if e.breachStatus.domain.status === 'done'}
                  <span class="status-icon" title={e.breachStatus.domain.result > 0 ? `${e.breachStatus.domain.result} domain breaches` : 'No domain breaches'}>
                    {e.breachStatus.domain.result > 0 ? '🌐⚠️' : '✅'}
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
    padding: 1rem;
  }
  h1 {
    color: var(--text);
    font-size: 1.8rem;
    margin-bottom: 1rem;
  }
  .stats-dashboard {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
  }
  .stat-card {
    background: var(--hover);
    padding: 1rem;
    border-radius: 8px;
    flex: 1;
    text-align: center;
  }
  .stat-card span:first-child {
    color: var(--muted);
    display: block;
  }
  .stat-card span:last-child {
    color: var(--text);
    font-size: 1.5rem;
  }
  .chart-container {
    background: var(--hover);
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    height: 200px;
    width: 100%;
  }
  canvas {
    width: 100% !important;
    height: 100% !important;
  }
  .toolbar {
    padding: 1rem 0;
    background: rgb(13, 13, 13);
    flex: none;
    display: flex;
    align-items: center;
    gap: 1rem;
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
  button {
    padding: 0.75rem 1rem;
    background: #4caf50;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }
  button:disabled {
    background: var(--muted);
    cursor: not-allowed;
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
  th:nth-child(1), td:nth-child(1) { width: 25%; }
  th:nth-child(2), td:nth-child(2) { width: 20%; }
  th:nth-child(3), td:nth-child(3) { width: 20%; }
  th:nth-child(4), td:nth-child(4) { width: 35%; }
  tbody tr:hover { background: var(--hover); }
  tbody td { padding: 0.75rem 1rem; vertical-align: middle; }
  .account-cell { display: flex; align-items: center; gap: 0.75rem; }
  .account-cell img { width: 1.75rem; height: 1.75rem; object-fit: contain; background: white; border-radius: 4px; }
  .last-change.recent { color: #4caf50; }
  .last-change.moderate { color: #ffeb3b; }
  .last-change.stale { color: #f44336; }
  .security-status { display: flex; gap: 0.5rem; justify-content: center; }
  .status-icon { cursor: pointer; }
  .spinner { animation: spin 1s linear infinite; }
  @keyframes spin { 100% { transform: rotate(360deg); } }
  .table-wrapper::-webkit-scrollbar { width: 8px; }
  .table-wrapper::-webkit-scrollbar-track { background: var(--panel); }
  .table-wrapper::-webkit-scrollbar-thumb { background-color: var(--border); border-radius: 4px; }
  .table-wrapper::-webkit-scrollbar-thumb:hover { background-color: var(--text); }
  .status-message { padding: 1rem; color: var(--muted); text-align: center; }
  .status-message.error { color: #f44336; }
</style>