# Qué probar al instalar DexGridCraft

Instala el APK (`DexGridCraft-…-arm64-v8a.apk` o el de debug de la CI) en un móvil o tableta con
Android 10 o posterior. Si algo falla, pulsa **Ctrl+Mayús+D** (o abre el icono «DexGridCraft
diagnostics» del APK de debug), copia el informe y pégalo en el issue o el chat.

Marca cada punto con ✅ / ❌ y anota lo que veas.

## 1. Arranque y cierre (DeX, con ratón)
- [ ] Abre la app desde el escritorio de DeX: aparece en una ventana de ~1280×800 con un libro vacío.
- [ ] Ciérrala con la X de la ventana y vuelve a abrirla. **Tres veces seguidas**, sin que se cierre
      sola, se cuelgue o quede en negro.
- [ ] Maximiza, redimensiona y mueve la ventana entre la pantalla del móvil y el monitor: la
      interfaz se redibuja a su tamaño y no se reinicia.
- [ ] El título de la ventana en la barra de tareas de DeX es «Libro1 — DexGridCraft» (o el nombre
      del archivo abierto) y añade «(edited)» al hacer cambios.
- [ ] Idioma: la interfaz sale en el idioma del sistema (español si el móvil está en español), y el
      tema claro/oscuro sigue al del sistema.

## 2. Ratón
- [ ] Clic en celdas, arrastrar para seleccionar un rango, doble clic para editar una celda.
- [ ] Clic derecho en una celda, en la cabecera de fila/columna y en la pestaña de la hoja: sale el
      menú contextual.
- [ ] Rueda: desplaza la hoja en vertical; Mayús+rueda en horizontal; Ctrl+rueda hace zoom.
- [ ] Cursor: flecha normal, mano sobre enlaces, I sobre la barra de fórmulas, flechas de
      redimensionar en los bordes de las cabeceras de columna (y arrastrar para cambiar el ancho).
- [ ] La cinta (Inicio, Insertar, Fórmulas…) cambia de pestaña y los botones responden al pasar el
      ratón por encima.

## 3. Teclado físico
- [ ] Escribe en una celda: letras, números, `ñ`, acentos (´ + a = á), `@`, `€` con AltGr.
- [ ] Enter / Tab / flechas mueven la celda activa; F2 edita; Supr borra; Esc cancela.
- [ ] Fórmulas: `=SUMA(A1:A3)` (o `=SUM(A1:A3)`), `=PROMEDIO(…)`, `=HOY()`; se recalculan al cambiar
      los datos.
- [ ] Atajos: Ctrl+Z / Ctrl+Y, Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+B (negrita), Ctrl+F (buscar).
- [ ] **Copiar y pegar**:
  - [ ] Ctrl+C en un rango y Ctrl+V en otro sitio de la hoja.
  - [ ] Copia texto en otra app (navegador, Notas) y pégalo con Ctrl+V en una celda.
  - [ ] Copia un rango de DexGridCraft y pégalo en otra app (aparece como texto con tabuladores).
  - [ ] Copia una tabla de una web o de otra hoja de cálculo y pégala: los números siguen siendo
        números.
  - [ ] Ctrl+X corta; el botón Pegar de la cinta pega lo que hay en el portapapeles del sistema.
  - [ ] Copia una imagen (por ejemplo de la Galería) y pégala con el botón Pegar: se inserta.

## 4. Archivos
- [ ] Archivo ▸ Abrir (y Ctrl+O): sale el selector de Android; abre un `.xlsx`. Se ve bien
      (formatos, anchos de columna, gráficos si los tiene).
- [ ] Abre también un `.csv`, un `.ods` y un `.xls` antiguo.
- [ ] Edita el `.xlsx` y pulsa Ctrl+S: **el cambio aparece en el archivo original** (ábrelo con otra
      app o vuelve a abrirlo).
- [ ] Libro nuevo ▸ Ctrl+S (o Archivo ▸ Guardar como ▸ Examinar): sale el selector para crear el
      archivo; tras elegirlo se guarda, el título cambia al nombre nuevo y el Backstage se cierra.
- [ ] Archivo ▸ Exportar ▸ Crear PDF, CSV y Página web: cada uno pide dónde guardarlo y el archivo
      aparece con su contenido.
- [ ] Desde «Mis archivos», «Abrir con» un `.xlsx` → DexGridCraft: se abre en la app.
- [ ] Desde otra app, «Compartir» un `.xlsx` o `.csv` → DexGridCraft: se abre.
- [ ] En DeX, arrastra un `.xlsx` desde «Mis archivos» a la ventana: se abre.
- [ ] Archivo ▸ Abrir ▸ Recientes: lista los abiertos y vuelven a abrirse.
- [ ] Insertar ▸ Imágenes ▸ Examinar: elige una imagen PNG/JPEG y se inserta en la hoja.
- [ ] Con AutoGuardado activado (barra de título) y un archivo abierto, los cambios se guardan solos.

## 5. Cerrar con cambios sin guardar
- [ ] Haz un cambio y pulsa la X de la ventana (o el botón Atrás): pregunta Guardar / No guardar /
      Cancelar. Cancelar deja la app abierta; No guardar la cierra.
- [ ] Guardar en un libro nuevo: sale el selector y, al elegir, se guarda (luego hay que cerrar otra
      vez; ver «límites» abajo).

## 6. Funciones del sistema
- [ ] Archivo ▸ Imprimir: aparece la impresora «Android»; al imprimir se abre el diálogo de impresión
      de Android con la vista previa del PDF; «Guardar como PDF» funciona.
- [ ] Un hipervínculo `https://…` en una celda (Ctrl+clic o clic según GridCraft) abre el navegador.
- [ ] Ayuda ▸ Buscar actualizaciones: dice que la app la actualiza su tienda (no intenta descargar).
- [ ] Textos en chino, japonés, coreano, árabe, hebreo y tailandés se ven con sus letras (no
      cuadrados ▯).
- [ ] Insertar ▸ Gráfico sobre un rango de datos: se dibuja el gráfico.

## 7. Móvil o tableta (sin DeX)
- [ ] Toca una celda y escribe: sale el teclado en pantalla, con predicción, y lo escrito va a la
      celda; el teclado no tapa la celda que editas.
- [ ] La interfaz no queda debajo de la barra de estado ni de la de navegación.
- [ ] Gira la pantalla: se adapta sin reiniciarse ni perder el libro.
- [ ] Desplazamiento con el dedo en la hoja; pellizcar para hacer zoom.
- [ ] Sal a la pantalla de inicio y vuelve: el libro sigue igual.

## 8. Rendimiento
- [ ] Un libro grande (miles de filas) se desplaza con fluidez.
- [ ] El informe de diagnóstico (Ctrl+Mayús+D) dice `renderer wgpu, … (Vulkan…)` o
      `renderer glow`; si dice glow, anótalo.

## Límites conocidos (no son fallos)
- «Guardar una copia» en `.xlsx` guarda el libro en el archivo elegido en lugar de dejar una copia
  aparte; la exportación a PDF desde el Backstage usa las opciones por defecto (ver
  [plan.md](plan.md)).
- Tras «Guardar» en la pregunta de cerrar con un libro sin archivo, la app no se cierra sola: vuelve
  a pulsar la X.
- Las ventanas de libro (Vista ▸ Nueva ventana) son internas, no ventanas de DeX aparte.
