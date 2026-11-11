# Plan y estado

DexGridCraft sigue el [prompt de portado de DexPdfCraft](https://github.com/manoloGith/DexPdfCraft/blob/main/docs/PORTING-PROMPT.md).
Como GridCraft usa el mismo egui (`6d518fc`) y el mismo winit (`a517b38`) que PdfCraft, toda la capa
Android viene hecha en craft-android y las fases se han hecho de una vez, sin pasar por el
dispositivo entre ellas. La lista de pruebas en el dispositivo está en
[pruebas.md](pruebas.md).

## Lo que se encontró al estudiar GridCraft (sección 6 del prompt)

| Punto | En GridCraft | En DexGridCraft |
|---|---|---|
| Arranque (`apps/gridcraft/src/main.rs`) | `SheetApp::new(Session, Services)`, `ui.json`/`prefs.json`, `DocWindows` (una ventana nativa por libro), canal de control, Apple events, autoactualización | `GridApp` (`src/lib.rs`) hace lo mismo sin `DocWindows`, sin canal de control, sin Apple events y sin autoactualización |
| `rfd` / `arboard` | Solo en el binario: la interfaz los recibe por `Services` | No hacen falta shims: `src/android/mod.rs` rellena `Services` con craft-android |
| `Command::new` | `fc-match` (fuente árabe preferida), `xdg-open` (en el binario) | `fc-match` falla sin más en Android; los enlaces van por `Services::open_url` → Intent |
| Viewports secundarios | Cada libro, una ventana nativa (`DocWindows`) | Sin `DocWindows`: las ventanas de libro son internas, como en la versión web |
| Entradas de compilación | `CRAFT_FONTS_DIR` opcional; las traducciones van en el código | Nada que descargar; fuentes de `/system/fonts` |
| Marcas | Logotipos de ArtCraft solo en `docs/brand/` (la interfaz no los incrusta) | Icono propio |
| Datos y configuración | `$XDG_CONFIG_HOME/gridcraft` en Linux | craft-android apunta XDG al almacenamiento privado |
| Formatos | xlsx, xlsm, xls, xlsb, ods, csv, tsv, txt, slk, json | Filtros de intents `VIEW`/`EDIT`/`SEND` con sus tipos MIME |
| Autoguardado | AutoGuardado propio (cada 2 s con cambios, si está activado) | Igual; además las preferencias se guardan al perder el foco |
| Renderizado | wgpu en escritorio; la interfaz no depende de wgpu | wgpu, con glow (OpenGL ES) de reserva |

## Fases

| Fase | Hecho |
|---|---|
| 0 | Repositorio, fork con rama `dexcraft/android`, submódulos `upstream` y `craft-android`, workspace con los `[patch]` de upstream copiados y winit de craft-android, Gradle + cargo-ndk, CI, NOTICE, licencias, icono propio |
| 1 | eframe en GameActivity (CraftActivity), wgpu con glow de reserva, el motor en arm64, diagnóstico de craft-android (renderizador, versión) |
| 2 | La interfaz real de GridCraft, arrancada como en escritorio; entorno HOME/XDG/LANG (craft-android); idioma y tema del sistema; fuentes de `/system/fonts` (fork) |
| 3 | Selectores del SAF con escritura de vuelta: Abrir, Guardar como, Exportar (CSV, HTML, PDF), Insertar imagen; portapapeles (texto, HTML, imagen) y Ctrl+C/X/V; «Abrir con» y «Compartir»; ratón, teclado y ciclo de vida (craft-android); informe de diagnóstico (Ctrl+Mayús+D, solo en el APK de debug desde 0.2.0) |
| 4 | Cursores, título de la tarea («Libro1 — DexGridCraft»), arrastrar y soltar libros (craft-android y `droppableMimeTypes`) |
| 5 | Imprimir: la página Imprimir de GridCraft lista una impresora «Android» y su PDF va al diálogo de impresión del sistema; enlaces con Intent; sin autoactualización |
| 6 | Teclado en pantalla e insets (craft-android) |
| 7 | Perfil de release (LTO, `codegen-units = 1`), firma y workflow de release, metadatos de F-Droid/Obtainium |

## Decisiones y límites conocidos

- **Los selectores no bloquean.** En GridCraft, `pick_open` y `pick_save` esperan la respuesta en el
  hilo de la interfaz. En Android eso cuelga la app, porque la pausa de la actividad (que ocurre al
  abrir el selector) espera a ese hilo. *Abrir* usa `Services::open_async`, que GridCraft ya tenía
  para la web; *Guardar como*, *Exportar* y *Guardar una copia* usan `Services::pick_save_async`, que
  añade el fork: GridCraft guarda lo que debe hacer y lo hace cuando llega la respuesta, con las
  mismas opciones que en escritorio (por ejemplo, el rango de la exportación a PDF).
- **Ctrl+C/X/V.** En Android, egui-winit no tiene portapapeles y las teclas llegan de la actividad
  como teclas sueltas. Lo resuelve el `Shell` de craft-android (`clipboard_keys`) para todas las apps.
- **Pestañas de libros (0.2.0).** Como en Android todas las ventanas de libro van dentro de la
  única ventana de la actividad, bajo la cinta hay una pestaña por ventana (`src/tabs.rs`, con la
  `TabStrip` de craft-android y el enganche `Services::below_ribbon` del fork). Cada acción usa los
  comandos de GridCraft: `view.switchWindows`, `window.close` (pregunta si hay cambios) y `file.new`.
- **Una sola ventana.** «Nueva ventana» y «Cambiar ventanas» funcionan dentro de la app, como en la
  versión web de GridCraft.
- **Sin impresoras propias.** El número de copias y la doble cara se eligen en el diálogo de Android.
