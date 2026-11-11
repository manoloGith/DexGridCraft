# El fork de GridCraft

`upstream/` es un submódulo de **[manoloGith/gridcraft](https://github.com/manoloGith/gridcraft)**,
un fork de `storytold/gridcraft`, en la rama **`dexcraft/android`**: el `main` de upstream (GridCraft
0.16.0) más dos commits que DexGridCraft necesita en Android.

| Cambio | Por qué |
|---|---|
| Fuentes de `/system/fonts` (`crates/ui-egui/src/theme/android_fonts.rs`, enganchado en `font_setup::candidates` y `font_fallback::fallback_faces`) | Android guarda ahí sus fuentes (Roboto, Noto CJK, árabe, hebreo, tailandés…) y no tiene fontconfig. Solo se compila para Android. |
| «Ayuda ▸ Buscar actualizaciones» en Android (`app/prefs.rs`) | La autoactualización de escritorio no sirve para una app Android: lo dice en lugar de ofrecerla. |
| Diálogo de guardar que responde más tarde (`Services::pick_save_async`, `app/save_picker.rs`) | Guardar como, Exportar y Guardar una copia esperaban la respuesta de `pick_save` en el hilo de la interfaz. En Android el selector es otra actividad y la pausa de la actividad espera a ese hilo: se colgaría. Con `pick_save_async` el host abre el selector y manda la ruta después; GridCraft guarda la acción pendiente y la ejecuta, igual que tras el diálogo de escritorio. Genérico y con tests (`tests/it/file_menu.rs`): se podría proponer a storytold. |

A diferencia de PdfCraft, **no hacen falta los shims de `rfd` y `arboard`**: la interfaz de GridCraft
recibe los diálogos de archivos y el portapapeles del binario que la arranca (`Services`), así que la
app Android le da los suyos (`apps/dexgridcraft/src/android/mod.rs`) sin tocar la interfaz.

## Trabajar en el fork

En `upstream/`, `origin` es el fork y `storytold` el proyecto original:

```bash
git -C upstream checkout dexcraft/android
```

Después de hacer commit en `upstream/`, sube la rama y registra aquí el nuevo commit del submódulo:

```bash
git -C upstream push origin dexcraft/android
```

```bash
git add upstream && git commit -m "upstream: …"
```

## Actualizar a una versión nueva de GridCraft

```bash
scripts/sync-upstream.sh
```

Rebasa `dexcraft/android` sobre el `main` de storytold, comprueba que los `[patch.crates-io]` de
este workspace siguen coincidiendo con los de upstream (y que winit sigue en la revisión de
craft-android) y vuelve a tomar su `Cargo.lock`. Luego: compilar, probar, subir la rama con
`--force-with-lease` y hacer commit del submódulo.
