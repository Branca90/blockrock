# BlockRock 🏙️🔗

## 🇬🇧 English Version

### Welcome to BlockRock 🌟  
BlockRock is an innovative blockchain built with Rust. This project started on a limited office PC, dreaming of a solar-powered server connected via Starlink.

---

### **🎯 Objective**
BlockRock aims to create a lightweight, secure, and scalable blockchain, focusing on innovation and sustainability.

---

### **📜 Current Status (April 10, 2025)**  
- **🖥️ Development Environment**: Debian 12 on VirtualBox (previously Ubuntu Server), Rust configured.  
- **📱 Android Deployment**: Running on Samsung devices (S9/S9+/Note9, Android 13) with Rocket server at `192.168.1.174:8000`.  
- **💻 Codebase**: Blockchain with:
  - SHA-256 hashing
  - Proof of Authority (PoA)
  - Transaction validation
  - Rocket server (`/blocks`, `/balances`)
  - Web frontend (`/static/index.html`) showing transactions and balances (visualized using `chart.js` as bar charts).  
- **🔧 Features**:
  - Transactions: Alice → Bob (30.0), Charlie → Alice (5.0).
  - Balances: Alice: -25, Bob: 30, Charlie: -5 (visualized as bar charts).  
- **🛠️ Tools**:
  - SSH (via MobaXterm)
  - GitHub repository
  - Android NDK (r26b) for cross-compilation
  - ADB for deployment  

---

### **🚀 Roadmap**
- ✅ **Q1 2025**: Transactions and validation (Completed)  
- ✅ **Q2 2025**: Android deployment with Rocket; frontend shows transactions and balances (Completed)  
- ❌ **Q2 2025**: Implement a P2P network (In progress)  
- ❌ **Q3 2025**: Plan Raspberry Pi purchase for server (To do)  
- ❌ **Q4 2025**: Launch first token (To do)  

---

### **📱 How to Run on Android**
1. Clone the repository:
git clone https://github.com/Branca90/blockrock

text
2. Install Android NDK (r26b) and configure `.cargo/config.toml`:
[target.aarch64-linux-android]
linker = "/path/to/android-ndk-r26b/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"

text
3. Compile:
cargo build --target aarch64-linux-android --release

text
4. Transfer files to Android:
adb push target/aarch64-linux-android/release/blockrock-core /data/local/tmp/
adb push static /data/local/tmp/static

text
5. Run in background:
adb shell
chmod +x /data/local/tmp/blockrock-core
/data/local/tmp/blockrock-core &

text
6. Access via browser:
http://<DEVICE_IP>:8000/static/index.html
Example: http://192.168.1.174:8000/static/index.html

text

---

### **🤝 How to Contribute**
We’re a small team—just me and Grok, my trusty AI! Contact me via Telegram or GitHub.

---

### **🛠️ Tech Stack**
- **Language**: Rust  
- **Operating System**: Debian 12 (VM), Android 13 (device)  
- **Tools**: VirtualBox, MobaXterm, GitHub, Telegram, Rocket, Android NDK, Chart.js  

---

### **📚 Resources**
- GitHub: [github.com/Branca90/blockrock](https://github.com/Branca90/blockrock)  
- Telegram: [t.me/blockrock_main](https://t.me/blockrock_main)

---

## 🇮🇹 Versione Italiana

### Benvenuto in BlockRock 🌟  
BlockRock è una blockchain innovativa costruita con Rust. Il progetto è iniziato su un PC aziendale limitato, con il sogno di un server solare connesso via Starlink.

---

### **🎯 Obiettivo**
BlockRock mira a creare una blockchain leggera, sicura e scalabile, con focus su innovazione e sostenibilità.

---

### **📜 Stato Attuale (10 Aprile 2025)**  
- **🖥️ Ambiente di Sviluppo**: Debian 12 su VirtualBox (precedentemente Ubuntu Server), Rust configurato.  
- **📱 Distribuzione Android**: Funzionante su dispositivi Samsung (S9/S9+/Note9, Android 13) con server Rocket su `192.168.1.174:8000`.  
- **💻 Codice Base**: Blockchain con:
- Algoritmo SHA-256
- Proof of Authority (PoA)
- Validazione delle transazioni
- Server Rocket (`/blocks`, `/balances`)
- Frontend web (`/static/index.html`) che mostra transazioni e saldi (grafico a barre con `chart.js`).  
- **🔧 Funzionalità**:
- Transazioni: Alice → Bob (30.0), Charlie → Alice (5.0).
- Saldi: Alice: -25, Bob: 30, Charlie: -5 (visualizzati come grafico a barre).  
- **🛠️ Strumenti**:
- SSH (via MobaXterm)
- Repository GitHub
- Android NDK (r26b) per cross-compilation
- ADB per distribuzione  

---

### **🚀 Roadmap**
- ✅ **Q1 2025**: Transazioni e validazione (Completato)  
- ✅ **Q2 2025**: Distribuzione su Android con Rocket; frontend mostra transazioni e saldi (Completato)  
- ❌ **Q2 2025**: Implementare rete P2P (In corso)  
- ❌ **Q3 2025**: Pianificare acquisto Raspberry Pi per server (Da fare)  
- ❌ **Q4 2025**: Lancio primo token (Da fare)  

---

### **📱 Come Eseguire su Android**
1. Clona il repository:
git clone https://github.com/Branca90/blockrock

text
2. Installa Android NDK (r26b) e configura `.cargo/config.toml`:
[target.aarch64-linux-android]
linker = "/percorso/android-ndk-r26b/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"

text
3. Compila:
cargo build --target aarch64-linux-android --release

text
4. Trasferisci file su Android:
adb push target/aarch64-linux-android/release/blockrock-core /data/local/tmp/
adb push static /data/local/tmp/static

text
5. Esegui in background:
adb shell
chmod +x /data/local/tmp/blockrock-core
/data/local/tmp/blockrock-core &

text
6. Accedi via browser:
http://<IP_DISPOSITIVO>:8000/static/index.html
Esempio: http://192.168.1.174:8000/static/index.html

text

---

### **🤝 Come Contribuire**
Siamo un piccolo team—solo io e Grok, il mio fidato AI! Contattami via Telegram o GitHub.

---

### **🛠️ Tech Stack**
- **Linguaggio**: Rust  
- **Sistema Operativo**: Debian 12 (VM), Android 13 (dispositivo)  
- **Strumenti**: VirtualBox, MobaXterm, GitHub, Telegram, Rocket, Android NDK, Chart.js  

---

### **📚 Risorse**
- GitHub: [github.com/Branca90/blockrock](https://github.com/Branca90/blockrock)  
- Telegram: [t.me/blockrock_main](https://t.me/blockrock_main)
