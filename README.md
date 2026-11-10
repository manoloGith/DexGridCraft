# DexGridCraft

La interfaz de escritorio de [GridCraft](https://github.com/storytold/gridcraft), la hoja de cálculo
de código abierto escrita en Rust, en Android. Pensada para **Samsung DeX** y para el modo de
ventanas de escritorio de Android (monitor, ratón y teclado); funciona también en tabletas y móviles.

No se reescribe la interfaz: se compila para Android la propia interfaz egui de GridCraft
(`gridcraft-ui-egui`) y se le pone debajo la capa Android compartida de los Crafting Apps
([craft-android](https://github.com/manoloGith/craft-android)), la misma que usa
[DexPdfCraft](https://github.com/manoloGith/DexPdfCraft).

> Estado: **0.1.0, fases 0 a 7 hechas sin probar aún en un dispositivo**. El APK compila; lo que hay
> que comprobar al instalarlo está en [docs/pruebas.md](docs/pruebas.md). Plan y decisiones en
> [docs/plan.md](docs/plan.md); los cambios a GridCraft, en [docs/fork.md](docs/fork.md).

## Cómo está hecho

```
DexGridCraft/
├── upstream/                 submódulo: fork manoloGith/gridcraft, rama dexcraft/android
├── craft-android/            submódulo: manoloGith/craft-android (capa Android + winit con W-DEX1..3)
├── apps/dexgridcraft/        la crate de la app (cdylib que carga Android) + binario de escritorio
│   ├── src/lib.rs            GridApp: arranca la interfaz de GridCraft como su binario de escritorio
│   ├── src/android/mod.rs    android_main y los Services de GridCraft sobre craft-android
│   ├── src/clipboard.rs      Ctrl+C/X/V y el portapapeles del sistema en Android
│   ├── src/prefs.rs          ui.json y prefs.json, donde los guarda GridCraft en Linux
│   └── android/              proyecto Gradle: MainActivity : CraftActivity
├── scripts/sync-upstream.sh  rebasa el fork sobre storytold/gridcraft
├── fastlane/metadata/        F-Droid / Obtainium
└── .github/workflows/        android.yml (CI), release.yml (firma + release)
```

## Compilar

Requisitos: `rustup` (instala la versión y los targets de `rust-toolchain.toml`),
`cargo install cargo-ndk --version 4.1.2 --locked`, Android SDK (plataforma 36, build-tools 36.0.0),
NDK 29.0.14206865 y JDK 17 o posterior.

```bash
git clone --recursive https://github.com/manoloGith/DexGridCraft
```

```bash
cd DexGridCraft/apps/dexgridcraft/android && ./gradlew assembleDebug -Pdexgridcraft.abis=arm64-v8a
```

El APK queda en `app/build/outputs/apk/debug/`. La biblioteca de Rust se compila optimizada también
en debug (la cuadrícula es inusable sin optimizar); `-Pdexgridcraft.rustDebug=true` la deja sin
optimizar.

Para iterar sin dispositivo, la misma ventana en escritorio (sin selectores ni portapapeles del
sistema):

```bash
cargo run -p dexgridcraft --features desktop -- libro.xlsx
```

Publicar una versión: [docs/releasing.md](docs/releasing.md). F-Droid y Obtainium:
[docs/fdroid.md](docs/fdroid.md).

## Licencia

MIT o Apache-2.0, como GridCraft. Avisos y marcas en [NOTICE.md](NOTICE.md).
