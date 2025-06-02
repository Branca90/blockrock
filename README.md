![image](https://github.com/user-attachments/assets/9d0c80d9-9e0a-4ae3-a34b-cc24dae69287)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Stars](https://img.shields.io/github/stars/Branca90/blockrock)](https://github.com/Branca90/blockrock)
[![Twitter](https://img.shields.io/twitter/follow/BlockRock572744?style=social)](https://twitter.com/BlockRock572744)

 **BlockRock** è una blockchain P2P open-source che trasforma il tuo smartphone in un nodo IoT. Scritta in Rust, gira su Samsung S9, raccoglie dati come GPS ed energia solare, e si integra con Tron per il token BRK. 

 ## Supporta il Progetto
Se ti piace Zion Core, considera di supportarlo tramite [GitHub Sponsors](https://github.com/sponsors/BlockRockAdmin)!

---

🌟 ## Cos'è BlockRock?

BlockRock è una piattaforma blockchain leggera, sicura e decentralizzata, sviluppata da zero in Rust.
Trasforma smartphone di uso comune (es. Samsung S9) in nodi IoT che raccolgono dati reali come GPS ed energia solare.
Questi nodi formano una rete peer-to-peer (P2P), contribuendo a un ecosistema sostenibile e guidato dalla community.

Integrazione:
BlockRock si integra con la blockchain Tron (Nile Testnet) tramite l’API REST TronGrid e mira a un’interoperabilità cross-chain futura attraverso ponti come Rubic o Router Nitro.

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
> Dettagli completi nel [Wiki - Guida Installazione]((https://github.com/BlockRockAdmin/BlockRock/wiki)).

---

## Come Contribuire

Vedi [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licenza

Vedi [LICENSE.md](LICENSE.md).

---

## Contatti

| Social | LINK |
| ------ | ------ |
| **Twitter:** | [@BlockRock572744](https://twitter.com/BlockRock572744) |
| **Telegram:** | [t.me/blockrock_main](https://t.me/blockrock_main) |

---

Un progetto folle con il sogno di server solari e Starlink! 🚀
