![image](https://github.com/user-attachments/assets/9d0c80d9-9e0a-4ae3-a34b-cc24dae69287)# BlockRock 🏙️

![BlockRock Logo]([https://pbs.twimg.com/media/Gm_u5sJXQAE6sXi?format=jpg&name=medium]) <!-- Aggiungi un logo se ce l'hai -->

[![Build Status](https://github.com/Branca90/blockrock/actions/workflows/rust.yml/badge.svg)](https://github.com/Branca90/blockrock/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Stars](https://img.shields.io/github/stars/Branca90/blockrock)](https://github.com/Branca90/blockrock)
[![Twitter](https://img.shields.io/twitter/follow/BlockRock572744?style=social)](https://twitter.com/BlockRock572744)

🌟 **BlockRock** è una blockchain P2P open-source che trasforma il tuo smartphone in un nodo IoT. Scritta in Rust, gira su Samsung S9, raccoglie dati come GPS ed energia solare, e si integra con Tron per il token BRK. Un progetto folle con il sogno di server solari e Starlink! 🚀

---

## Cos'è BlockRock?

BlockRock è una blockchain leggera e sicura che usa smartphone come nodi IoT per raccogliere dati reali.  
Si integra con Tron per tokenizzazione a basso costo e punta a un futuro di interoperabilità cross-chain e sostenibilità.

### Funzionalità Principali

- **Rete P2P**: Consenso Proof of Authority (PoA)
- **IoT**: Supporto GPS (prototipo), sensori solari in sviluppo
- **Tron**: Token BRK in arrivo (Q3 2025)
- **Gamification**: Ispirata a personaggi come **P** (Lies of P) e **Vash** (Trigun)

Per approfondire, visita il [Wiki](https://github.com/Branca90/blockrock/wiki).

---

## Stato Attuale (26 Aprile 2025)

- **Ambiente**: Debian 12, Rust 1.76+
- **Android**: Samsung S9/S9+/Note9 (Android 13)
- **Core**: Hashing SHA-256, PoA, validazione transazioni
- **IoT**: GPS attivo, sensori solari in corso
- **Tron**: Integrazione in sviluppo

---

## Installazione

1. **Clona il Repository**:
git clone https://github.com/Branca90/blockrock.git
cd blockrock

2. **Imposta la Chiave TronGrid**:
```export TRONGRID_API_KEY="your-trongrid-api-key"```

3. **Compila ed Esegui**:

```
cargo build --release
cargo run --release
```

4. **Accedi**:
```
[http://localhost:8000/static/index.html](http://localhost:8000/static/index.html)
```
> Dettagli completi nel [Wiki - Guida Installazione](https://github.com/Branca90/blockrock/wiki/Guida-Installazione).

---

## Come Contribuire

Scopri di più, Vedi [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licenza

Sotto licenza MIT. Vedi [LICENSE.md](LICENSE.md).

---

## Contatti

- **Twitter:** [@BlockRock572744](https://twitter.com/BlockRock572744)
- **Telegram:** [t.me/blockrock_main](https://t.me/blockrock_main)

---
