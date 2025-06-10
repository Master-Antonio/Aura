# Aura - System Performance Optimizer

![Aura Logo](./src/assets/aura-icon.svg)

**Aura** è un'applicazione desktop avanzata per il monitoraggio delle prestazioni di sistema e l'ottimizzazione automatica per il gaming e l'uso intensivo, progettata per Windows, Linux e macOS. Combina una moderna interfaccia React/Material-UI con un backend Tauri/Rust ad alte prestazioni.

---

## Caratteristiche principali

- **Monitoraggio in tempo reale** di CPU, GPU, RAM, Storage, Network
- **Gestione avanzata dei processi**: kill, sospendi, riprendi, cambia affinità CPU, gaming boost
- **Ottimizzazioni di sistema** con un click: power plan, Game Mode, trasparenze, servizi, privacy, ecc.
- **Ottimizzazioni specifiche per gaming** (P-cores, priorità, Game DVR, GameMode Linux, ecc.)
- **Interfaccia drag & drop** per personalizzare la dashboard
- **Supporto multi-GPU** e dettagli hardware
- **Compatibilità multipiattaforma** (Windows, Linux, macOS)
- **UI moderna**

---

## Architettura

- **Frontend**: React 19, TypeScript, Material-UI 7, Vite
- **Backend**: Tauri 2.5, Rust, accesso nativo a API di sistema
- **Comunicazione**: API Tauri commands tra frontend e backend
- **Struttura**:
  - `src/` → Frontend React
  - `src-tauri/` → Backend Rust/Tauri

---

## Installazione

### Prerequisiti
- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)
- [PNPM](https://pnpm.io/) (o npm/yarn)

### Setup
```sh
pnpm install
pnpm tauri dev
```

Per buildare la versione finale:
```sh
pnpm build
pnpm tauri build
```

---

## Uso

- **Dashboard**: Visualizza in tempo reale l'utilizzo di CPU, GPU, RAM, Storage, Network
- **Tabella processi**: Gestisci i processi (kill, sospendi, riprendi, info, gaming boost)
- **Gaming Boost**: Ottimizza un processo per il gaming (priorità, affinità P-cores, ecc.)
- **Ottimizzazioni**: Applica/ripristina ottimizzazioni di sistema con un click
- **Impostazioni**: Personalizza intervallo di refresh, layout, ecc.

---

## Ottimizzazioni disponibili
- **Windows**: Game DVR, Game Mode, High Performance Power Plan, trasparenze, animazioni, privacy, servizi
- **Linux**: GameMode, CPU governor, swappiness, compositor, kernel params
- **macOS**: Spotlight, servizi, ottimizzazioni RAM
- **Universali**: High priority, clear memory/dns cache, ecc.

---

## Sviluppo

- **Frontend**:
  - `pnpm dev` per avviare solo il frontend
- **Backend**:
  - Modifica in `src-tauri/src/` (Rust)
  - Comandi custom in `src-tauri/src/commands/`

---

## Contribuire

Pull request e segnalazioni sono benvenute!

---

## Licenza

Il codice sorgente di Aura è consultabile, ma **non può essere utilizzato, copiato o distribuito al di fuori di questo repository** senza autorizzazione esplicita dell'autore. I contributi sono sempre benvenuti tramite pull request.

Licenza: **Proprietaria con permesso di consultazione e contributo (source-available, contribution welcome)**

---

## Autori

- [anton](https://github.com/anton) e collaboratori

## Supportami
paypal.me/MasterAntonio
