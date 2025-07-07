import { load as loadStore } from "@tauri-apps/plugin-store";
import { locale } from '../language';
import { language } from '../stores';
import { get } from 'svelte/store';

// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

export async function load() {
  // Only load from store if locale is not already set (i.e., on initial app load)
  const store = await loadStore("settings.json");
  const savedLanguage = await store.get("language");

  const currentLang = savedLanguage ? (savedLanguage as string) : 'en';
  language.set(currentLang);

  return {};
}
