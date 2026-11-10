# Avisos

DexGridCraft usa el motor y la interfaz de **GridCraft**
(<https://github.com/storytold/gridcraft>, © 2026 ArtCraft Team and the GridCraft contributors), con
licencia **MIT o Apache-2.0**, incluidos como submódulo en `upstream/` desde el fork
[manoloGith/gridcraft](https://github.com/manoloGith/gridcraft) (rama `dexcraft/android`, con dos
cambios pequeños para Android: ver `docs/fork.md`). Los avisos y licencias de terceros de GridCraft
están en `upstream/NOTICE` y `upstream/ATTRIBUTION.md`, y se aplican también aquí.

La capa Android (entrada, ciclo de vida, documentos, portapapeles, impresión y winit con sus
parches) viene del submódulo `craft-android/`
([manoloGith/craft-android](https://github.com/manoloGith/craft-android), MIT o Apache-2.0; su
`winit/` conserva la licencia Apache-2.0 de winit).

El proceso de compilación con Gradle y cargo-ndk viene de
[DexPdfCraft](https://github.com/manoloGith/DexPdfCraft), que a su vez parte del de
[PdfCraft for Android](https://github.com/artcraft-android/pdfcraft-android) (MIT o Apache-2.0).

## Marcas

El nombre y los logotipos de **ArtCraft** (`upstream/docs/brand/`) son marcas del ArtCraft Team y
**no** son de código abierto: no pueden usarse en versiones modificadas
(`upstream/docs/brand/LICENSE-brand.txt`). DexGridCraft no incluye ni muestra esos logotipos: la
interfaz de GridCraft no los incrusta, y la app usa un icono propio (una cuadrícula sobre un
monitor, en verde). El icono de GridCraft no es el icono de la app.

El código propio de DexGridCraft se distribuye con la misma licencia doble, MIT o Apache-2.0.
