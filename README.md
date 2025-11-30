# Cryptor — Outil de chiffrement en Rust

## Cryptor est un programme écrit en Rust, il permet de chiffrer, déchiffrer et encoder des fichiers.

---

## Fonctionnalités

Cryptor prend en charge :

### 1. Chiffrement

- **AES-256-GCM**  
  (sécurisé, standard industriel)

- **ChaCha20-Poly1305**  
  (sécurisé, rapide, moderne)

- **XOR**  
  (non sécurisé, purement pédagogique)

### 2. Déchiffrement

- Inverse exact des modes ci-dessus  
- Gestion des erreurs (mot de passe incorrect, fichier corrompu…)

### 3. Base64

- Encodage  
- Décodage

---

## Usage

Les commandes s’exécutent via :

```bash
cargo run -- <commande> [options]

```
Chiffrement AES

```bash

cargo run -- encrypt --algo aes --input input.txt --output out.bin --password exemple

```
Déchiffrement AES
```bash
cargo run -- encrypt --algo aes --input input.txt --output out.bin --password exemple
```
Chiffrement ChaCha20
```bash
cargo run -- decrypt --algo aes --input out.bin --output result.txt --password exemple

```
Mode XOR
```bash
cargo run -- encrypt --algo xor --input input.txt --output out.bin
```
Encodage Base64
```
cargo run -- encode --algo base64 --input input.txt --output encoded.txt
```
Décodage Base64
```
cargo run -- decode --algo base64 --input encoded.txt --output decoded.bin
```
----
## Format des fichiers chiffrés (AES / ChaCha20)

Les fichiers chiffrés suivent le format :
```
[SALT     : 16 octets]
[NONCE    : 12 octets]
[CIPHERTEXT + TAG]
```
----
## Architecture du projet

```
src/
├── main.rs            Point d’entrée du programme
├── cli.rs             Gestion du parsing des arguments (Clap)
├── io.rs              Lecture et écriture de fichiers
|
└── crypto/
    ├── aes.rs         Chiffrement AES-256-GCM
    ├── chacha.rs      Chiffrement ChaCha20-Poly1305
    ├── xor.rs         Chiffrement XOR (pédagogique)
    ├── base64.rs      Encodage/Décodage Base64
    └── mod.rs         Module global regroupant les crypto
```
----
### Notes importantes

- Le dossier target/ ne contient que les fichiers compilés, il ne fait pas partie du code source.

- Les fichiers .bin sont volontairement illisibles (c’est du binaire).

- Un mot de passe incorrect rend le déchiffrement impossible.

- AES et ChaCha20 utilisent PBKDF2 + Salt + Nonce pour une sécurité robuste.








