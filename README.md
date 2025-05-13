**Aplicație de Administrare și Auditare Parole**

Această documentație descrie modul de instalare, configurare și utilizare al aplicației de auditare și management al parolelor, dezvoltată pe Tauri, SvelteKit, TypeScript și Rust.

---

## Cuprins

1. [Introducere](#introducere)
2. [Funcționalități principale](#functionalitati-principale)
3. [Arhitectură și Tehnologii](#arhitectura-si-tehnologii)
4. [Instalare și Configurare](#instalare-si-configurare)

   * 4.1. Cerințe preliminare
   * 4.2. Clonare și configurare mediu
   * 4.3. Construire (build)
   * 4.4. Rulare în mod dezvoltare
5. [Ghid de Utilizare](#ghid-de-utilizare)

   * 5.1. Scanare parole în browsere și directoare
   * 5.2. Generare One-Time Codes (OTP)
   * 5.3. Salvare și gestionare `Recovery Keys`
   * 5.4. Monitorizare parole slabe și recomandări de schimbare
6. [Detalii de Securitate](#detalii-de-securitate)
7. [Structura Proiectului](#structura-proiectului)
8. [Contribuții și Licență](#contribu%C8%9Bii-si-licenta)

---

## Introducere

Aplicația are rolul de a extrage și centraliza parolele stocate în browserele instalate pe sistem, precum și din folderele **Desktop** și **Documents**, oferind:

* Audit de securitate: identifică parolele slabe sau reutilizate.
* Generator OTP (One-Time Passwords).
* Stocare securizată a `Recovery Keys` pentru codurile OTP.
* Alerta utilizatorului în cazul parolelor compromise și sugestii de modificare.

Această soluție a fost creată pentru competiția Info-Educația.

---

## Funcționalități principale

* **Scanare parole în browsere**: extrage și afișează parolele salvate în Google Chrome, Firefox, Edge etc.
* **Scanare directoare**: detectează fișiere cu extensii uzuale (.txt, .docx, .json etc.) care conțin șiruri ce pot semăna cu parole.
* **Generator OTP**: generează coduri unice de autentificare conform standardului TOTP.
* **Stocare `Recovery Keys`**: salvează și criptează cheile de recuperare OTP în spațiul securizat al aplicației.
* **Audit parole**: analizează lungimea și complexitatea parolelor, afișează scor și recomandă modificări cu link direct către pagina de reset/parolă.

---

## Arhitectură și Tehnologii

* **Tauri**: cadru pentru desktop-app multi-platformă.
* **SvelteKit**: frontend rapid și reactiv.
* **TypeScript**: siguranță la tipuri în codul UI.
* **Rust**: logică cryptografică și interacțiunea cu fișierele de sistem, pentru performanță și securitate.

---

## Instalare și Configurare

### 4.1. Cerințe preliminare

* **Node.js** >= 16.x
* **npm** sau **pnpm**
* **Rust** și `cargo`
* `tauri-cli`: `cargo install tauri-cli`

### 4.2. Clonare și configurare mediu

```bash
# Clonare repository
git clone https://github.com/utilizator/aplicatie-parole.git
cd aplicatie-parole

# Instalare dependențe frontend
pnpm install  # sau npm install
```

### 4.3. Construire (build)

```bash
# Build frontend și backend Tauri\ npm run build
ta release
```

### 4.4. Rulare în mod dezvoltare

```bash
# Pornește serverul de dezvoltare SvelteKit
pnpm dev
# În alt terminal, pornește Tauri dev
pnpm tauri dev
```

---

## Ghid de Utilizare

### 5.1. Scanare parole în browsere și directoare

1. Deschide aplicația desktop.
2. Accesează secțiunea **„Scanare parole”**.
3. Alege dacă vrei să scanezi:

   * Parole din browsere instalate.
   * Directoarele `Desktop` și `Documents`.
4. Apasă **„Start Scan”**.
5. Rezultatele apar sub formă de listă cu detalii: URL/Fișier, utilizator, număr de caractere, scor complexitate.

### 5.2. Generare One-Time Codes (OTP)

1. Navighează la **„OTP Generator”**.
2. Adaugă un nou secret (ex: cod QR sau manual).
3. Aplicația va afișa coduri dinamice valabile 30s.

### 5.3. Salvare și gestionare `Recovery Keys`

* La crearea unui nou secret OTP, aplicația va genera și afișa un `Recovery Key`.
* Cheia de recuperare este criptată și stocată local, accesibilă doar cu parola principală a aplicației.

### 5.4. Monitorizare parole slabe și recomandări de schimbare

* După scanare, parolele cu scor sub un prag (ex. 60/100) sunt evidențiate.
* Pentru fiecare parolă slabă, se afișează un buton **„Schimbă parola”** cu link către pagina de schimb relevantă.

---

## Detalii de Securitate

* **Criptare AES-256** pentru stocarea fișierului de baze de date locale.
* **Zero-knowledge**: datele de autentificare nu părăsesc niciodată dispozitivul.
* **Rust** asigură memoria sigură și evita vulnerabilități C/C++.

---

## Structura Proiectului

```
├── src-tauri/           # Backend Rust și configurație Tauri
│   ├── main.rs         
│   └── ...             
├── src/                 # SvelteKit frontend
│   ├── routes/         
│   ├── lib/            
│   └── app.html        
├── package.json        
├── tauri.conf.json     
└── README.md           
```

---

## Contribuții și Licență

* Pentru contribuții, deschide un *Pull Request* pe GitHub.
* Respectă *code style* și adaugă teste pentru funcționalități noi.
* Licență: **MIT**.

---

*Documentație generată pentru concursul Info-Educația 2025.*
