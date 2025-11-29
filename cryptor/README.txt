Cryptor — Outil de chiffrement en Rust

Cryptor est un programme en ligne de commande écrit en Rust.
Il permet de chiffrer, déchiffrer et encoder des fichiers à l’aide d’algorithmes modernes ou pédagogiques.

Le but du projet est de proposer une architecture propre, modulaire et extensible tout en illustrant l’implémentation de plusieurs techniques cryptographiques.

Fonctionnalités

Cryptor prend en charge :

• Chiffrement

AES-256-GCM (sécurisé, standard industriel)

ChaCha20-Poly1305 (sécurisé, rapide, moderne)

XOR (non sécurisé, purement pédagogique)

• Déchiffrement

Inverse exact des modes ci-dessus

Gestion d’erreurs (mot de passe incorrect, fichier corrompu…)

• Base64

Encodage

Décodage

Usage général

Toutes les commandes suivent la même structure :

cargo run -- <commande> [options]


Les commandes disponibles sont :

encrypt      Chiffrer un fichier
decrypt      Déchiffrer un fichier
encode       Encoder en Base64
decode       Décoder du Base64

Exemples d’utilisation
Chiffrement AES
cargo run -- encrypt --algo aes --input test.txt --output out.bin --password bobo

Déchiffrement AES
cargo run -- decrypt --algo aes --input out.bin --output result.txt --password bobo

Chiffrement ChaCha20
cargo run -- encrypt --algo chacha --input test.txt --output out.bin --password secret

XOR (sans mot de passe)
cargo run -- encrypt --algo xor --input test.txt --output out.bin

Base64

Encoder :

cargo run -- encode --algo base64 --input test.txt --output encoded.txt


Décoder :

cargo run -- decode --algo base64 --input encoded.txt --output decoded.bin

Format des fichiers chiffrés

Pour AES et ChaCha20, le fichier chiffré est structuré :

[16 octets → SALT]
[12 octets → NONCE]
[ciphertext + tag → données chiffrées]


Ces valeurs aléatoires garantissent sécurité, non-rejeu et unicité des chiffrages.

Pour XOR, il s’agit simplement du flux XOR du fichier (méthode non sécurisée).

Architecture du code
src/
│
├── main.rs          Point d'entrée, exécution du CLI
├── cli.rs           Gestion des commandes (Clap)
├── io.rs            Lecture / écriture de fichiers
│
└── crypto/
    ├── aes.rs       Implémentation AES-256-GCM + PBKDF2
    ├── chacha.rs    Implémentation ChaCha20-Poly1305 + PBKDF2
    ├── xor.rs       Chiffrement XOR simple
    ├── base64.rs    Encodage et décodage Base64
    └── mod.rs       Regroupement des modules


AES et ChaCha utilisent PBKDF2-HMAC-SHA256 avec :

Salt aléatoire (16 octets)

Nonce aléatoire (12 octets)

100 000 itérations

Clés dérivées de longueur 256 bits

Points importants

Si le mot de passe est incorrect, la décryption échoue volontairement.

Le dossier target/ ne fait pas partie du code (Résultats de compilation uniquement).

Les fichiers out.bin et result.txt ne sont que des exemples.