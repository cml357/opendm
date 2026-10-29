# 🚀 Guide de Compilation et d'Installation d'OpenDM (Windows)

OpenDM est un gestionnaire de téléchargement accéléré développé en Rust avec interface Tauri v2.

---

## 1. Prérequis Windows
- **Rust (MSVC toolchain)** : [https://rustup.rs](https://rustup.rs)
- **Visual Studio C++ Build Tools**
- **Node.js** (v18+)

---

## 2. Compilation en une commande

```bash
# Vérifier l'ensemble du workspace OpenDM
cargo check --workspace

# Compiler le binaire final optimisé
cargo build --release
```

Le binaire exécutable sera disponible dans :
`target/release/opendm.exe`

---

## 3. Lancer l'interface Tauri

```bash
cd crates/opendm-tauri
npm install
npm run tauri dev
```

---

## 4. Fonctionnalités Spéciales d'OpenDM
- **Limiteur de bande passante** (`crates/opendm-core/src/limiter.rs`) : Régulation Token Bucket en direct et Mode Silencieux.
- **Muxer & Extracteur Audio pur Rust** (`crates/opendm-muxer`) : Assemblage MP4 et extraction AAC/MP3 sans FFmpeg ni yt-dlp.
- **Extensions Firefox & Chrome** (`extension-firefox/` et `extension-chrome/`) : Reniflage automatique des vidéos et envoi vers `opendm.exe`.
