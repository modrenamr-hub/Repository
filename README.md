# Musahih Pro

Musahih Pro is a commercial-ready desktop assistant that detects and corrects Arabic/English keyboard layout mistakes system-wide on Windows.

## Highlights
- Tauri + Rust architecture optimized for low memory and CPU usage.
- Layered correction engine with confidence scoring.
- Freemium licensing with online verification stub and offline grace period.
- Privacy-first implementation: no keystroke logging, no typed text persistence.
- Premium UI shell with notification and settings windows.

## Monorepo Structure
- `src-tauri/` — Rust core/runtime for detection, correction, licensing, storage and services.
- `ui/` — Frontend app shell (notification/settings/tray views).
- `license-server-stub/` — Example API for license validation.
- `docs/` — Privacy policy, packaging guide, commercial launch checklist.

## Quick Start
### Prerequisites
- Rust stable (1.77+ recommended)
- Node.js 20+
- Tauri prerequisites for Windows build (WebView2, Visual Studio Build Tools)

### Backend checks
```bash
cd src-tauri
cargo test
cargo bench --no-run
```

### Frontend
```bash
cd ui
npm install
npm run build
```

### Desktop run (when Tauri deps are installed)
```bash
npm run tauri dev
```

## Build + Packaging (Windows)
1. Build frontend assets: `cd ui && npm run build`.
2. Build signed installer (configure certificate first):
   ```bash
   cd src-tauri
   cargo tauri build
   ```
3. Build portable flavor by adding a second bundle target in `tauri.conf.json`.

## Security & Privacy Design
- Password field protection hooks are represented in listener policy and must be wired to Win32 UI Automation in production.
- Buffering is in-memory only; no typed phrase persistence.
- Logging excludes sensitive payload by design.
- License secrets loaded via environment (`MUSAHIH_LICENSE_PUBLIC_KEY`, API URL).

## Deliverables Included
1. Full project structure
2. Source files for core, storage, services, and UI shell
3. Build instructions
4. Packaging guidance
5. License server stub
6. Professional README
7. Privacy policy draft
8. Commercial launch checklist

## Commercial Readiness Notes
This repository is intentionally production-oriented and modular. Integrating full Windows low-level hooks and code-signing certificates is environment-dependent and should be done on your release machine.
