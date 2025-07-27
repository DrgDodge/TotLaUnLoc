import { writable } from 'svelte/store';

export const welcomeComplete = writable(false);
export const language = writable('en');
export const licenseKey = writable('');
export const machineId = writable('');
export const theme = writable('');
