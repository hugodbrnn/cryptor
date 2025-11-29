Cryptor — Outil de chiffrement en Rust

Cryptor est un programme en ligne de commande écrit en Rust.
Il permet de chiffrer, déchiffrer et encoder des fichiers à l’aide d’algorithmes modernes ou pédagogiques.

Le projet propose une architecture modulaire, propre et extensible illustrant plusieurs techniques cryptographiques contemporaines.

Fonctionnalités

Cryptor prend en charge :

1. Chiffrement

AES-256-GCM
(sécurisé, standard industriel)

ChaCha20-Poly1305
(sécurisé, rapide, moderne)

XOR
(non sécurisé, purely pédagogique)

2. Déchiffrement

Inverse exact des modes ci-dessus

Gestion des erreurs (mot de passe incorrect, fichier corrompu…)

3. Base64

Encodage

Décodage

Usage

Toutes les commandes s’utilisent sous la forme :

cargo run -- <commande> [options]

Chiffrement AES
cargo run -- encrypt --algo aes --input input.txt --output out.bin --password exemple

Déchiffrement AES
cargo run -- decrypt --algo aes --input out.bin --output result.txt --password exemple

Chiffrement ChaCha20
cargo run -- encrypt --algo chacha --input input.txt --output out.bin --password secret

Mode XOR
cargo run -- encrypt --algo xor --input input.txt --output out.bin

Encodage Base64
cargo run -- encode --algo base64 --input input.txt --output encoded.txt

Décodage Base64
cargo run -- decode --algo base64 --input encoded.txt --output decoded.bin

Format des fichiers chiffrés (AES / ChaCha20)

Les fichiers chiffrés suivent le format :

[SALT     : 16 octets]
[NONCE    : 12 octets]
[CIPHERTEXT + TAG]


Le salt et le nonce sont générés aléatoirement pour chaque opération.

Architecture du projet
src/
├── main.rs            Point d’entrée du programme
├── cli.rs             Gestion des commandes et arguments (Clap)
├── io.rs              Lecture et écriture de fichiers
│
└── crypto/
    ├── aes.rs         AES-256-GCM
    ├── chacha.rs      ChaCha20-Poly1305
    ├── xor.rs         XOR pédagogique
    ├── base64.rs      Encodage/Décodage Base64
    └── mod.rs         Regroupe les modules cryptographiques

Notes

Le dossier target/ n’a aucun impact sur le code source : il ne contient que les compilations.

Les fichiers chiffrés .bin sont volontairement illisibles.

Un mot de passe incorrect empêche tout déchiffrement.
