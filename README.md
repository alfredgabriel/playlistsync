# 🎵 PlaylistSync
### Sincronizador y descargador de playlists de audio en alta calidad
*Desktop application to import playlist CSVs and download high-quality audio with embedded metadata.*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Tauri v2](https://img.shields.io/badge/Built%20with-Tauri%20v2-24C8D8?logo=tauri&logoColor=white)](https://tauri.app/)
[![Svelte v5](https://img.shields.io/badge/Svelte-v5-FF3E00?logo=svelte&logoColor=white)](https://svelte.dev/)
[![Rust Backend](https://img.shields.io/badge/Rust-Backend-black?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![yt-dlp Bundled](https://img.shields.io/badge/yt--dlp-Bundled-red)](https://github.com/yt-dlp/yt-dlp)
[![Brutalist UI](https://img.shields.io/badge/UI-Brutalist%20Monochrome-white)](https://github.com/alfredgabriel/playlistsync)

---

![PlaylistSync Preview](PlaylistSync.png)

---

## 🇪🇸 Español

### 1. Visión y Propósito

**PlaylistSync** es una aplicación de escritorio diseñada para convertir tus listas de reproducción exportadas en archivos de audio de alta fidelidad, listos para tu reproductor local, coche o biblioteca offline.

Importa cualquier archivo `.csv` generado por herramientas como **Exportify** (Spotify) o **TuneMyMusic**, busca automáticamente las mejores coincidencias en YouTube Music y descarga cada pista etiquetándola con su metadato oficial (título, artista, álbum y carátula).

Todo funciona de forma autónoma: **no necesitas instalar Python, ni FFmpeg ni yt-dlp** en tu sistema operativo, ya que vienen empaquetados internamente como binarios nativos (sidecars de Tauri).

---

### 🚀 Uso Rápido

#### 1. Exportar tu lista a CSV
- Para Spotify: entra en [Exportify](https://exportify.net/) y descarga el `.csv` de tu playlist.
- Para otras plataformas (Apple Music, Tidal, Deezer): usa [TuneMyMusic](https://www.tunemymusic.com/transfer).

#### 2. Cargar en PlaylistSync
1. Abre la aplicación y entra en la pestaña **DOWNLOAD**.
2. Arrastra el archivo `.csv` a la zona de carga o haz clic para seleccionarlo.
3. Comprueba la previsualización de las canciones detectadas.

#### 3. Configurar y Descargar
1. Elige la carpeta destino en tu disco donde se guardará la música.
2. Selecciona el formato de audio:
   - **M4A (AAC 192kbps)** — Calidad recomendada, menor peso y máxima compatibilidad.
   - **MP3 (hasta 320kbps / VBR0)** — Compatibilidad universal.
3. (Opcional) Activa la generación del archivo `.m3u` para importar la playlist completa en reproductores como VLC, foobar2000 o Poweramp.
4. Pulsa **START DOWNLOAD →** y observa el progreso pista a pista.

---

### 🛠️ Desarrollo y Compilación

#### Requisitos
- [Node.js](https://nodejs.org/) v18+
- [Rust](https://www.rust-lang.org/tools/install) y herramientas de compilación de C++ (MSVC en Windows)

#### Ejecutar en desarrollo
```bash
npm install
npm run tauri dev
```

#### Compilar ejecutable de producción (.exe / instalador)
```bash
npm run tauri build
```
El instalador y el ejecutable standalone se generarán en:
`src-tauri/target/release/bundle/`

---

### 🛡️ Características Principales

- ⚡ **Sidecars autónomos**: `yt-dlp` y `ffmpeg` preempaquetados; no requiere configuración en el PATH del sistema.
- 🏷️ **Metadatos completos**: Inyección automática de ID3v2/MP4 tags (Artista, Título, Álbum, Número de pista).
- 🌐 **Multilingüe**: Interfaz disponible en Español, Inglés, Francés y Alemán.
- 🖤 **Estética Brutalista**: Diseño minimalista de alto contraste en blanco y negro, optimizado para legibilidad y rendimiento.

---

## 🇬🇧 English

### 1. Overview & Purpose

**PlaylistSync** is a high-performance desktop application designed to bridge the gap between streaming playlists and your offline audio library.

Import any standard `.csv` export from **Exportify** (Spotify) or **TuneMyMusic**, automatically locate matches on YouTube Music, and download high-bitrate audio with comprehensive metadata tags embedded directly into each file.

**No external dependencies required**: yt-dlp and ffmpeg are bundled directly within the app as native Tauri sidecars.

---

### 🚀 Step-by-Step Guide

1. **Export Playlist:** Export your playlist to `.csv` using [Exportify](https://exportify.net/) or [TuneMyMusic](https://www.tunemymusic.com/transfer).
2. **Import:** Drag & drop the `.csv` file into PlaylistSync under the **DOWNLOAD** tab.
3. **Configure:** Pick your destination folder, preferred format (M4A or MP3), and playlist file options (`.m3u`).
4. **Execute:** Click **START DOWNLOAD →** to download and tag every track with real-time feedback.

---

### 🛠️ Development & Build

#### Prerequisites
- [Node.js](https://nodejs.org/) v18+
- [Rust](https://www.rust-lang.org/tools/install)

#### Run locally (dev mode)
```bash
npm install
npm run tauri dev
```

#### Build standalone release
```bash
npm run tauri build
```

---

## 📜 License

Distributed under the **MIT License**. See `LICENSE` for details.
