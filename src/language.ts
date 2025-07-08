import { writable, derived } from 'svelte/store';

export const locale = writable('en');

interface TranslationKeys {
  [key: string]: string;
}

interface Translations {
  [locale: string]: TranslationKeys;
}

const translations: Translations = {
  en: {
    "welcome_title": "Welcome to TotLaUnLoc!",
    "welcome_subtitle": "The last account manager you will ever need.",
    "get_started": "Get Started",
    "select_language": "Select Language",
    "enter_license_key": "Enter License Key",
    "enter_your_license_key": "Enter your license key",
    "submit": "Submit",
    "skip": "Skip",
    "passwords": "Passwords",
    "one_time_codes": "One-Time Codes",
    "check_passwords": "Check Passwords",
    "settings": "Settings",
    "theme": "Theme",
    "light": "Light",
    "dark": "Dark",
    "auto_lock_after_inactivity": "Auto-lock after inactivity",
    "minutes": "minutes",
    "close": "Close",
    "search": "Search",
    "account": "Account",
    "username": "Username",
    "code": "Code",
    "invalid_otpauth_url": "Invalid otpauth URL.",
    "failed_to_add_account": "Failed to add account: ",
    "failed_to_delete_account": "Failed to delete account: ",
    "enter_otpauth_url": "Enter otpauth URL (e.g., otpauth://totp/user?secret=XXX&issuer=Service)",
    "add_account": "Add Account",
    "oldest_first": "Oldest First",
    "recent_first": "Recent First",
    "loading": "Loading...",
    "failed_to_load_passwords": "Failed to load passwords.",
    "no_browsers_found": "No browsers found.",
    "days_ago": "{days} days ago",
    "months_ago": "{months} months ago",
    "years_ago": "{years} years ago",
    "confirm_delete": "Confirm Delete",
    "confirm_delete_profile_message": "Are you sure you want to delete profile \"{profileName}\" and all its passwords?",
    "cancel": "Cancel",
    "delete": "Delete",
    "confirm_delete_password_message": "Are you sure you want to delete the password for \"{url}\" ({username})?",
    "total_accounts": "Total Accounts",
    "weak_passwords": "Weak Passwords",
    "breached_accounts": "Breached Accounts",
    "on_breached_domains": "On Breached Domains",
    "checking": "Checking...",
    "check_for_breaches": "Check for Breaches",
    "no_passwords_found": "No passwords found.",
    "last_password_change": "Last Password Change",
    "security_status": "Security Status",
    "breaches_found": "{count} breaches found",
    "no_breaches": "No breaches",
    "domain_breaches": "{count} domain breaches",
    "no_domain_breaches": "No domain breaches",
    "activate": "Activate",
    "license_key": "License Key",
  },
  ro: {
    "welcome_title": "Bun venit la TotLaUnLoc!",
    "welcome_subtitle": "Ultimul manager de conturi de care veți avea nevoie vreodată.",
    "get_started": "Începeți",
    "select_language": "Selectați limba",
    "enter_license_key": "Introduceți cheia de licență",
    "enter_your_license_key": "Introduceți cheia de licență",
    "submit": "Trimite",
    "skip": "Omite",
    "passwords": "Parole",
    "one_time_codes": "Coduri unice",
    "check_passwords": "Verificați parolele",
    "settings": "Setări",
    "theme": "Temă",
    "light": "Luminos",
    "dark": "Întunecat",
    "auto_lock_after_inactivity": "Blocare automată după inactivitate",
    "minutes": "minute",
    "close": "Închide",
    "search": "Căutați",
    "account": "Cont",
    "username": "Nume de utilizator",
    "code": "Cod",
    "invalid_otpauth_url": "URL otpauth invalid.",
    "failed_to_add_account": "Eroare la adăugarea contului: ",
    "failed_to_delete_account": "Eroare la ștergerea contului: ",
    "enter_otpauth_url": "Introduceți URL-ul otpauth (ex: otpauth://totp/utilizator?secret=XXX&issuer=Serviciu)",
    "add_account": "Adăugați cont",
    "oldest_first": "Cel mai vechi primul",
    "recent_first": "Cel mai recent primul",
    "loading": "Se încarcă...",
    "failed_to_load_passwords": "Nu s-au putut încărca parolele.",
    "no_browsers_found": "Nu s-au găsit browsere.",
    "days_ago": "{days} zile în urmă",
    "months_ago": "{months} luni în urmă",
    "years_ago": "{years} ani în urmă",
    "confirm_delete": "Confirmați ștergerea",
    "confirm_delete_profile_message": "Sigur doriți să ștergeți profilul \"{profileName}\" și toate parolele sale?",
    "cancel": "Anulează",
    "delete": "Șterge",
    "confirm_delete_password_message": "Sigur doriți să ștergeți parola pentru \"{url}\" ({username})?",
    "total_accounts": "Total conturi",
    "weak_passwords": "Parole slabe",
    "breached_accounts": "Conturi compromise",
    "on_breached_domains": "Pe domenii compromise",
    "checking": "Se verifică...",
    "check_for_breaches": "Verificați breșele",
    "no_passwords_found": "Nu s-au găsit parole.",
    "last_password_change": "Ultima modificare a parolei",
    "security_status": "Stare de securitate",
    "breaches_found": "{count} breșe găsite",
    "no_breaches": "Fără breșe",
    "domain_breaches": "{count} breșe de domeniu",
    "no_domain_breaches": "Fără breșe de domeniu",
    "activate": "Activați",
    "license_key": "Cheie de licență",
  },
};

export const t = derived(locale, ($locale) => (key: string, vars?: Record<string, string | number>) => {
  let text = key;
  if (translations[$locale] && translations[$locale][key]) {
    text = translations[$locale][key];
  }

  if (vars) {
    for (const [k, v] of Object.entries(vars)) {
      text = text.replace(`{${k}}`, String(v));
    }
  }
  return text;
});
