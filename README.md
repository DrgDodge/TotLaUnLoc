# **TotLaUnLoc**

**Cod sursa:** 
```
https://github.com/DrgDodge/TotLaUnLoc.git
```

**Documentatie:**
https://docs.google.com/document/d/1HOPQRNpzEzPcdt7Ma_Uq_eCK-m01Qx9MOTAeGv0YngI/edit?usp=sharing

Imaginile folosite se pot gasi in folderul `static/icons`

**Dependențe cheie:**

Aplicația TotLaUnLoc este construită folosind un stack tehnologic modern, având la bază **Svelte** pentru interfața utilizator și **Rust** pentru logica de backend, toate integrate într-o aplicație desktop nativă prin intermediul framework-ului **Tauri**.

Pe lângă aceste tehnologii fundamentale, proiectul utilizează următoarele dependențe importante:

**Frontend (Svelte/JavaScript):**
- `@tauri-apps/api`: Interacțiunea cu API-urile Tauri.
- `@tauri-apps/plugin-opener`: Deschide link-uri externe.
- `@tauri-apps/plugin-shell`: Execută comenzi shell.
- `@tauri-apps/plugin-store`: Gestionează stocarea datelor locale.
- `buffer`: Manipularea datelor binare.
- `chart.js`: Generarea graficelor statistice.
- `otpauth`: Generarea și validarea codurilor TOTP/HOTP.
- `otplib`: Biblioteca pentru autentificare OTP.
- `zxcvbn`: Estimarea forței parolelor.

**Backend (Rust):**
- `base32`: Codificare/decodificare Base32.
- `chrono`: Manipularea datelor și orelor.
- `humantime`: Formatarea timpului într-un format lizibil.
- `rusqlite`: Interacțiunea cu baza de date SQLite.
- `serde`, `serde_json`: Serializare și deserializare JSON.
- `tauri`: Framework-ul principal pentru aplicația desktop.
- `tauri-plugin-opener`: Plugin Tauri pentru deschiderea link-urilor.
- `tauri-plugin-shell`: Plugin Tauri pentru comenzi shell.
- `tauri-plugin-store`: Plugin Tauri pentru stocarea datelor.
- `tempfile`: Crearea fișierelor temporare.
- `totp-rs`: Generarea și validarea codurilor TOTP.
- `url`: Parsarea și manipularea URL-urilor.

## Cerințe de sistem

*   **Sistem de operare suportate în prezent:** Windows
*   **Sistem de operare suportate în viitor:** macOS, Linux (proiectul este bazat pe Tauri)
*   **Memorie RAM:** Minim 1GB (recomandat 2GB sau mai mult)
*   **Spațiu pe disc:** Minim 200MB spațiu liber
*   **Procesor:** Procesor dual-core sau mai bun
*   **Dependențe pentru compilare:**
    *   Node.js (pentru dezvoltare frontend)
    *   Rust (pentru backend-ul Tauri)
    *   NPM sau Yarn (pentru gestionarea pachetelor frontend)

## Descrierea proiectului

TotLaUnLoc este o aplicație desktop sigură și ușor de utilizat, concepută pentru a gestiona și stoca în siguranță parolele, codurile de autentificare unică (TOTP) și pentru a verifica dacă conturile dumneavoastră au fost compromise în breșe de date cunoscute. Este important de subliniat că aplicația nu accesează și nu decriptează parolele stocate de browsere. În schimb, TotLaUnLoc vă permite să gestionați conturile salvate pe majoritatea browserelor, oferind funcționalități precum ștergerea individuală a unui cont cu parola aferentă sau ștergerea tuturor parolelor dintr-un profil de browser. Aplicația oferă o interfață modernă, intuitivă și ușor de utilizat pentru organizarea credențialelor și asigură confidențialitatea datelor prin criptare locală.

**Notă:** Aplicația desktop TotLaUnLoc este complet gratuită pentru utilizatorii individuali. Pentru organizații și administratori de rețea care doresc să gestioneze zeci sau chiar sute de calculatoare simultan, este disponibil un panou web dedicat pentru achiziționarea și gestionarea licențelor: [TotLaUnLoc-panel](https://github.com/DrgDodge/TotLaUnLoc-panel). (**Inca in dezvoltare**)

## Descriere tehnică

Aplicația este construită folosind un stack tehnologic modern, combinând un frontend bazat pe Svelte cu un backend robust scris în Rust, toate integrate într-o aplicație desktop cu ajutorul framework-ului Tauri.

*   **Frontend:** Dezvoltat cu Svelte, un framework JavaScript reactiv care permite crearea de interfețe utilizator rapide și eficiente. Interfața este concepută pentru a fi intuitivă și responsivă.
*   **Backend:** Implementat în Rust, un limbaj de programare cunoscut pentru siguranța, performanța și concurența sa. Backend-ul gestionează logica de afaceri, inclusiv criptarea datelor, interacțiunea cu baza de date SQLite și generarea codurilor TOTP.
*   **Tauri:** Utilizează Tauri pentru a împacheta aplicația web (Svelte) într-o aplicație desktop nativă, oferind acces la funcționalități specifice sistemului de operare și o amprentă redusă a resurselor.
*   **Criptare:** Toate datele sensibile sunt criptate local pentru a asigura confidențialitatea și securitatea maximă.
*   **Verificare breșe de date:** Este important de menționat că aplicația **nu accesează și nu decriptează parolele stocate de browsere**. Funcționalitatea de verificare a breșelor de date se realizează prin compararea numelui site-ului și a contului/username-ului cu o bază de date locală de breșe cunoscute, asigurând astfel confidențialitatea parolelor dumneavoastră. Nicio parolă nu este trimisă către servere externe în timpul acestui proces.
*   **TOTP:** Integrarea cu biblioteca `totp-rs` permite generarea și gestionarea codurilor de autentificare unică bazate pe timp.