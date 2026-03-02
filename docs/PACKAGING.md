# Packaging and Release Guide

## Windows Installer
1. Configure code signing certificate in Windows cert store.
2. Set `TAURI_PRIVATE_KEY` and `TAURI_KEY_PASSWORD` for updater signature.
3. Run `cargo tauri build` from `src-tauri/`.
4. Verify NSIS/MSI output with SmartScreen reputation strategy.

## Portable Distribution
- Add portable target profile in Tauri bundle settings.
- Zip release with checksum and signature files.

## Bundle Optimization
- Use release profile with LTO.
- Strip debug symbols.
- Serve minified frontend bundles.
