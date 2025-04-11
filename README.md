
# BlockRock 🏙️🔗

[![Stars](https://img.shields.io/github/stars/Branca90/blockrock)](https://github.com/Branca90/blockrock)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Twitter](https://img.shields.io/twitter/follow/BlockRockIoT?style=social)](https://twitter.com/BlockRockIoT)

🌟 **BlockRock**: Una blockchain P2P open-source che trasforma il tuo smartphone in un nodo IoT! Costruita con Rust, gira su Samsung S9, raccoglie dati come GPS ed energia solare, e si integra con Tron per il token BRK. Un progetto folle nato su un PC aziendale, con il sogno di server solari e Starlink. 🚀

![BlockRock in Action](demo.gif)

---

## 🇬🇧 English Version

### What is BlockRock?
BlockRock is a lightweight, secure blockchain built from scratch in Rust. It started as a passion project on a limited office PC, with a vision of sustainability—think solar-powered servers and Starlink connectivity. Now, it’s evolving into a P2P network where smartphones (like my S9) act as IoT nodes, collecting GPS, solar energy, and more, all tied to Tron for low-cost tokenization.

### 🎯 Objective
Create a decentralized, innovative platform for IoT and blockchain, running on everyday devices. Low-cost, scalable, and community-driven—sustainability meets tech!

### 📜 Current Status (April 11, 2025)
- **🖥️ Dev Environment**: Debian 12 on VirtualBox, Rust configured.
- **📱 Android Deployment**: Running on Samsung S9/S9+/Note9 (Android 13) via Rocket server at `192.168.1.174:8000`.
- **💻 Core Features**:
  - SHA-256 hashing.
  - Proof of Authority (PoA) consensus.
  - Transaction validation (e.g., Alice → Bob: 30.0, Charlie → Alice: 5.0).
  - Rocket backend with endpoints `/blocks` (transaction list) and `/balances` (JSON balances: Alice: -25, Bob: 30, Charlie: -5).
  - Web frontend (`/static/index.html`) with bar charts (Chart.js) for balances.
- **🌍 IoT Progress**: GPS support added (prototype), solar energy sensors in development.
- **🔗 Tron Integration**: In progress—token BRK and IoT data on Tron blockchain (Q3 2025).
- **📡 Connectivity**: Tested with Starlink for remote P2P!

### 🚀 Roadmap
| Quarter       | Progress                             | Status       |
|---------------|-------------------------------------|-------------|
| Q1 2025       | Transaction validation and backend   | ✅ Completed |
| Q2 2025       | Android deployment and GPS prototype | ✅ Completed |
| Q2 2025       | P2P network implementation           | ⏳ In progress |
| Q3 2025       | Tron integration and Raspberry Pi    | ⏳ Planned   |
| Q4 2025       | Solar energy sensors and token BRK   | 🕒 Upcoming |

---

## 🇮🇹 Versione Italiana

### Cos’è BlockRock?
BlockRock è una blockchain leggera e sicura scritta in Rust. Nato su un PC aziendale limitato, sogna un futuro sostenibile con server solari e Starlink. Oggi è una rete P2P che usa smartphone (es. il mio S9) come nodi IoT, raccogliendo GPS, energia solare e integrandosi con Tron per il token BRK.

### 🎯 Obiettivo
Creare una piattaforma decentralizzata per IoT e blockchain su dispositivi quotidiani. Economica, scalabile e guidata dalla comunità—tecnologia e sostenibilità insieme!

### 📜 Stato Attuale (11 Aprile 2025)
- **🖥️ Ambiente di sviluppo**: Debian 12 su VirtualBox con Rust configurato.
- **📱 Distribuzione Android**: Funziona su Samsung S9/S9+/Note9 (Android 13), server Rocket su `192.168.1.174:8000`.
- **💻 Funzionalità principali**:
  - Hashing SHA-256.
  - Consenso Proof of Authority (PoA).
  - Validazione transazioni (es. Alice → Bob: 30.0, Charlie → Alice: 5.0).
  - Backend Rocket con endpoint `/blocks` (lista transazioni) e `/balances` (saldi JSON).
  - Frontend web (`/static/index.html`) con grafici a barre (Chart.js) per i saldi.
- **🌍 IoT**: Supporto GPS aggiunto (prototipo), sensori solari in sviluppo.
- **🔗 Integrazione Tron**: Token BRK e dati IoT sulla blockchain Tron in corso (Q3 2025).
- **📡 Connettività**: Testato con Starlink per rete P2P remota!

---

## 📱 Come Eseguire su Android

1. **Clona il repository**:
git clone https://github.com/Branca90/blockrock

text

2. **Configura NDK**:
Installa Android NDK r26b e modifica `.cargo/config.toml`:
[target.aarch64-linux-android]
linker = "/path/to/android-ndk-r26b/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"

text

3. **Compila l’applicazione**:
cargo build --target aarch64-linux-android --release

text

4. **Distribuisci i file su Android**:
adb push target/aarch64-linux-android/release/blockrock-core /data/local/tmp/
adb push static /data/local/tmp/static

text

5. **Esegui l’applicazione sul dispositivo**:
adb shell chmod +x /data/local/tmp/blockrock-core
adb shell /data/local/tmp/blockrock-core &


6. **Accedi al frontend tramite browser**:
Apri `http://<IP_DISPOSITIVO>:8000/static/index.html`  
Esempio: `http://192.168.1.174:8000/static/index.html`

---

## 🤝 Come Contribuire

BlockRock è un progetto solitario—solo io e Grok, il mio AI fidato! Ma ogni aiuto è benvenuto! Se sei interessato a Rust, IoT o Tron, contattaci:

- Twitter: [@BlockRockIoT](https://twitter.com/BlockRockIoT)
- Telegram: [t.me/BlockRockCommunity](https://t.me/BlockRockCommunity)

---

## 🛠️ Tech Stack

| Linguaggio      | Sistema Operativo     | Strumenti                     |
|-----------------|-----------------------|-------------------------------|
| Rust            | Debian12 - Android.13 | VirtualBox, Rocket, Chart.js  |

---

## 📄 Licenza

BlockRock è distribuito sotto licenza MIT ([LICENSE.md](LICENSE.md)). Progetto sperimentale—nessun dato personale raccolto.

---

Se hai bisogno di ulteriori modifiche o aggiunte specifiche fammi sapere!
