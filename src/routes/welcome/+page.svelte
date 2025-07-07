<script lang="ts">
  import { onMount } from "svelte";
  import { load } from "@tauri-apps/plugin-store";
  import { welcomeComplete, language, licenseKey } from '../../stores';
  import { type Store } from "@tauri-apps/plugin-store";
  import { fade } from 'svelte/transition';
  import { t, locale } from '../../language';

  let store: Store;
  let step = 1;
  let languages = [{name: 'English', flag: '/icons/english_flag.svg', langCode: 'en'}, {name: 'Romanian', flag: '/icons/romanian_flag.svg', langCode: 'ro'}];
  let selectedLanguage = 'English';
  let key = '';
  let welcomeText = "";
  let subtitleText = "";
  let buttonText = "";
  let selectLanguageTitle = "";
  let selectLanguageInterval: NodeJS.Timeout | null = null;
  let serverCheckStarted = false;

  // Server list with real addresses
  let servers = [
    { name: "Main license server", url: "https://h.lseb.top", status: "pending" },
    { name: "Secondary license server", url: "https://home.mimidev.top", status: "pending" },
    { name: "Backup Server", url: "https://invalid-url-for-testing.xyz", status: "pending" },
  ];

  // This reactive block will trigger the server check when step becomes 3
  $: if (step === 3 && !serverCheckStarted) {
    serverCheckStarted = true;
    pingServers();
  }

  onMount(async () => {
    store = await load("settings.json");
    // Welcome message typewriter logic remains the same...
    const welcomeMessages = ["Welcome to TotLaUnLoc!", "Bun venit la TotLaUnLoc!"];
    const subtitleMessages = ["The last account manager you will ever need.", "Ultimul manager de conturi de care veți avea nevoie vreodată."];
    const buttonMessages = ["Get Started", "Începeți"];
    let currentMessageIndex = 0;
    let isDeleting = false;
    let charIndex = 0;
    subtitleText = subtitleMessages[0];
    buttonText = buttonMessages[0];
    const typeWriter = () => {
      const fullWelcomeText = welcomeMessages[currentMessageIndex];
      if (isDeleting) {
        welcomeText = fullWelcomeText.substring(0, charIndex - 1);
        charIndex--;
      } else {
        welcomeText = fullWelcomeText.substring(0, charIndex + 1);
        charIndex++;
      }
      let typeSpeed = 100;
      if (isDeleting) typeSpeed /= 2;
      if (!isDeleting && charIndex === fullWelcomeText.length) {
        typeSpeed = 2000;
        isDeleting = true;
      } else if (isDeleting && charIndex === 0) {
        isDeleting = false;
        currentMessageIndex = (currentMessageIndex + 1) % welcomeMessages.length;
        typeSpeed = 500;
        subtitleText = subtitleMessages[currentMessageIndex];
        buttonText = buttonMessages[currentMessageIndex];
      }
      setTimeout(typeWriter, typeSpeed);
    };
    setTimeout(typeWriter, 100);
  });

  async function pingServers() {
    for (let i = 0; i < servers.length; i++) {
        try {
            // Use 'no-cors' mode for pinging, as we only need to check reachability
            await fetch(servers[i].url, { method: 'HEAD', mode: 'no-cors' });
            servers[i].status = "success";
        } catch (error) {
            // A catch block indicates the server is unreachable
            servers[i].status = "error";
        } finally {
            // Update the array reactively
            servers = [...servers];
            // A small delay for visual pacing
            if (i < servers.length -1) {
                await new Promise(resolve => setTimeout(resolve, 300));
            }
        }
    }
  }

  function nextStep() {
    step++;
    if (step === 2) {
      if (selectLanguageInterval) {
        clearTimeout(selectLanguageInterval);
        selectLanguageInterval = null;
      }
      // Language selection typewriter logic remains the same...
      const selectLanguageMessages = ["Select Language", "Selectați limba"];
      let currentSelectLanguageIndex = 0;
      let isSelectLanguageDeleting = false;
      let selectLanguageCharIndex = 0;
      const selectLanguageTypeWriter = () => {
        const fullSelectLanguageText = selectLanguageMessages[currentSelectLanguageIndex];
        if (isSelectLanguageDeleting) {
          selectLanguageTitle = fullSelectLanguageText.substring(0, selectLanguageCharIndex - 1);
          selectLanguageCharIndex--;
        } else {
          selectLanguageTitle = fullSelectLanguageText.substring(0, selectLanguageCharIndex + 1);
          selectLanguageCharIndex++;
        }
        let typeSpeed = 100;
        if (isSelectLanguageDeleting) typeSpeed /= 2;
        if (!isSelectLanguageDeleting && selectLanguageCharIndex === fullSelectLanguageText.length) {
          typeSpeed = 2000;
          isSelectLanguageDeleting = true;
        } else if (isSelectLanguageDeleting && selectLanguageCharIndex === 0) {
          isSelectLanguageDeleting = false;
          currentSelectLanguageIndex = (currentSelectLanguageIndex + 1) % selectLanguageMessages.length;
          typeSpeed = 500;
        }
        selectLanguageInterval = setTimeout(selectLanguageTypeWriter, typeSpeed);
      };
      selectLanguageTypeWriter();
    } else if (selectLanguageInterval) {
        clearTimeout(selectLanguageInterval);
        selectLanguageInterval = null;
    }
  }

  async function selectLanguage(langName: string) {
    selectedLanguage = langName;
    const langCode = languages.find(l => l.name === langName)?.langCode || 'en';
    language.set(langCode);
    await store.set("language", langCode);
    await store.save();
    nextStep();
  }

  async function submitLicense() {
    licenseKey.set(key);
    await store.set("licenseKey", key);
    await store.set("welcomeComplete", true);
    await store.save();
    welcomeComplete.set(true);
    window.location.reload();
  }

  async function skipLicense() {
    await store.set("welcomeComplete", true);
    await store.save();
    welcomeComplete.set(true);
    window.location.reload();
  }
</script>

<div class="welcome-container">
  {#if step === 1}
    <div class="welcome-step-content" in:fade>
      <h1 class="title" in:fade>{welcomeText}</h1>
      {#key subtitleText}
        <p class="subtitle" in:fade>{subtitleText}</p>
      {/key}
      {#key buttonText}
        <button class="button is-primary" on:click={nextStep}><span in:fade>{buttonText}</span></button>
      {/key}
    </div>
  {/if}

  {#if step === 2}
    <div class="welcome-step-content" in:fade>
      <h2 class="title">{selectLanguageTitle}</h2>
      <div class="language-selector">
        {#each languages as lang}
          <button class="button" on:click={() => selectLanguage(lang.name)}>
            <img src={lang.flag} alt={lang.name} class="flag-icon" />
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if step === 3}
    <div class="welcome-step-content" in:fade>
      {#key $language}
        <h2 class="title" in:fade>{$t('enter_license_key')}</h2>

        <div class="server-status-list">
          {#each servers as server}
            <div class="server-status-item">
              <span>{server.name}</span>
              <div class="status-icon">
                {#if server.status === 'pending'}
                  <div class="loader"></div>
                {:else if server.status === 'success'}
                  <div class="checkmark"></div>
                {:else if server.status === 'error'}
                  <div class="cross"></div>
                {/if}
              </div>
            </div>
          {/each}
        </div>

        <input class="input" type="text" bind:value={key} placeholder={$t('enter_your_license_key')} />

        <div class="button-group">
          <button class="button is-primary" on:click={submitLicense}><span in:fade>{$t('submit')}</span></button>
          <button class="button" on:click={skipLicense}><span in:fade>{$t('skip')}</span></button>
        </div>
      {/key}
    </div>
  {/if}
</div>

<style>
  .welcome-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background-color: #0d0d0d;
    color: #e0e0e0;
  }
  .welcome-step-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 2rem;
    background-color: #191919;
    border-radius: 12px;
    text-align: center;
    min-width: 400px;
    max-width: 600px;
    width: 90%;
  }
  .title {
    font-size: 2rem;
    font-weight: 700;
    min-height: 64px;
    margin-bottom: 0.5rem;
  }
  .subtitle {
    font-size: 1.1rem;
    color: #a0a0a0;
  }
  .language-selector {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }
  .button {
    padding: 0.75rem 1.5rem;
    border: 1px solid #444;
    border-radius: 8px;
    background: rgba(68, 68, 68, 0.2);
    color: #e0e0e0;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 1rem;
  }
  .button:hover {
    background: rgba(68, 68, 68, 0.4);
  }
  .button.is-primary {
    background-color: #4caf50;
    border-color: #4caf50;
    color: white;
  }
  .input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 8px;
    background: #141414;
    color: #e0e0e0;
    font-size: 1.1rem;
    box-sizing: border-box;
  }
  .button-group {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-top: 0.5rem;
  }
  .flag-icon {
    width: 100px;
    height: auto;
  }

  /* Server Status Styles */
  .server-status-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem; /* Reduced gap */
    align-items: stretch;
    width: 100%;
    margin-bottom: 1rem; /* Space before the input box */
  }
  .server-status-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.6rem 1rem; /* Slightly smaller padding */
    background-color: #222;
    border-radius: 8px;
    font-size: 1rem; /* Slightly smaller font */
  }
  .status-icon {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Loader Animation */
  .loader {
    border: 3px solid #f3f3f3;
    border-top: 3px solid #4caf50;
    border-radius: 50%;
    width: 20px;
    height: 20px;
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  /* Checkmark and Cross Styles */
  .checkmark::after {
    content: '✔';
    color: #4caf50;
    font-size: 24px;
  }
  
  .cross::after {
    content: '✖';
    color: #f44336;
    font-size: 24px;
  }
</style>