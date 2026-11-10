//! DexGridCraft: GridCraft's desktop interface on Android, for Samsung DeX and Android's desktop
//! windowing. The Android app (`android/`) is a `GameActivity` (craft-android's `CraftActivity`)
//! that loads this library and calls `android_main`; egui draws the whole window with wgpu
//! (Vulkan or GLES), or with OpenGL ES through glow when wgpu can't start.
//!
//! The window is GridCraft's own interface ([`SheetApp`]), as on the desktop, in one window: the
//! workbook windows the desktop opens as native windows are GridCraft's in-app windows here (as
//! in its web version), since an Android activity has one window.

pub mod clipboard;
pub mod prefs;

#[cfg(target_os = "android")]
pub(crate) mod android;

use std::sync::mpsc::{Receiver, Sender, channel};

use gridcraft_engine::Session;
use gridcraft_ui_egui::{Services, SheetApp};
use serde_json::json;

/// What to start with.
#[derive(Clone, Debug, Default)]
pub struct Launch {
    /// Workbooks to open (paths).
    pub files: Vec<String>,
}

/// A system picker's answer that arrives after the frame that asked for it. Android's pickers
/// are other activities: the interface thread can't wait for them, because the activity's
/// pause waits for that thread.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Picked {
    /// File › Open: the workbook to open.
    Open(String),
    /// Save As and the exports: where, and the format asked for (the suggested name's extension:
    /// `xlsx`, `csv`, `html` or `pdf`).
    Save { path: String, format: String },
}

/// Where the host's pickers send their late answers ([`Picked`]).
#[derive(Clone)]
pub struct Answers {
    tx: Sender<Picked>,
    ctx: egui::Context,
}

impl Answers {
    pub fn send(&self, picked: Picked) {
        let _ = self.tx.send(picked);
        self.ctx.request_repaint();
    }
}

/// The format a Save picker was asked for, from the name it suggests (`Book1.pdf` → `pdf`).
pub fn format_of(suggested: &str) -> String {
    let ext = std::path::Path::new(suggested).extension().and_then(std::ffi::OsStr::to_str).map(str::to_ascii_lowercase);
    match ext.as_deref() {
        Some(e @ ("csv" | "html" | "pdf")) => e.to_owned(),
        _ => "xlsx".to_owned(),
    }
}

/// The command that writes `format` to `path`, as the desktop's Save As and Export do once their
/// picker has answered.
pub fn save_command(path: &str, format: &str) -> (&'static str, serde_json::Value) {
    let id = match format {
        "pdf" => "file.exportPdf",
        "csv" => "file.exportCsv",
        "html" => "file.exportHtml",
        _ => "file.save",
    };
    let path = match (format, std::path::Path::new(path).extension()) {
        // A workbook saved without an extension is an .xlsx, as on the desktop.
        ("xlsx", None) => format!("{path}.xlsx"),
        _ => path.to_owned(),
    };
    (id, json!({ "path": path }))
}

/// GridCraft's interface as an eframe app: what the desktop binary's `App` does, minus the
/// native windows, the macOS menu bar and the control channel.
pub struct GridApp {
    pub sheet: SheetApp,
    answers: Receiver<Picked>,
}

impl GridApp {
    /// GridCraft as its desktop binary starts it (`upstream/apps/gridcraft/src/main.rs`), with the
    /// host's `services`, which get where to send their pickers' late answers.
    pub fn new(cc: &eframe::CreationContext<'_>, launch: &Launch, services: impl FnOnce(Answers) -> Services) -> GridApp {
        let (tx, answers) = channel();
        let mut sheet = SheetApp::new(Session::new(), services(Answers { tx, ctx: cc.egui_ctx.clone() }));
        sheet.watch_desktop_theme();
        prefs::load(&mut sheet);
        // Stamp the startup workbook's author from the persisted user name.
        if sheet.session.documents().is_empty() {
            sheet.session.new_workbook();
        }
        sheet.after_engine();
        for f in &launch.files {
            sheet.open_path(f);
        }
        SheetApp::setup_context_for_language(&cc.egui_ctx, sheet.ui.dark, sheet.ui.language);
        GridApp { sheet, answers }
    }

    /// The window title DeX shows in its taskbar: the workbook's, as GridCraft's title bar.
    pub fn title(&self) -> String {
        match self.sheet.session.active() {
            Some(d) => format!("{}{} — DexGridCraft", d.display_title(), if d.is_dirty() { " (edited)" } else { "" }),
            None => "DexGridCraft".to_owned(),
        }
    }

    /// Open a workbook as File › Open does (also "Open with" and dropped files).
    pub fn open(&mut self, path: &str) {
        self.sheet.open_path(path);
    }

    /// The pickers' late answers, applied as the desktop applies them when its picker returns.
    fn take_answers(&mut self) {
        while let Ok(picked) = self.answers.try_recv() {
            match picked {
                Picked::Open(path) => self.open(&path),
                Picked::Save { path, format } => {
                    if !self.sheet.commit_edit(0, 0, false, false) {
                        continue;
                    }
                    let (id, params) = save_command(&path, &format);
                    match self.sheet.run(id, params) {
                        // Done: the Backstage view (Save As, Export) closes, as on the desktop.
                        Ok(_) => self.sheet.backstage = None,
                        Err(e) => self.sheet.message = Some(("GridCraft".into(), gridcraft_ui_egui::clean_error(&e))),
                    }
                    self.sheet.after_engine();
                }
            }
        }
    }

    pub fn save_prefs(&self) {
        prefs::save(&self.sheet);
    }
}

impl eframe::App for GridApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.take_answers();
        #[cfg(target_os = "android")]
        for path in craft_android::android::events::opened() {
            self.open(&path);
        }
        self.sheet.logic(ctx);
        // As on the desktop: a print job holds back the close.
        if ctx.input(|input| input.viewport().close_requested()) && self.sheet.printing.busy() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        // Android: Ctrl+C, X and V arrive as plain keys; egui-winit would have made them
        // Copy, Cut and Paste with the system clipboard's text.
        #[cfg(target_os = "android")]
        clipboard::keys_to_clipboard_events(&mut raw.events, || match craft_android::android::clipboard::get() {
            Some(craft_android::android::clipboard::Clip::Text(text) | craft_android::android::clipboard::Clip::Html { text, .. }) => Some(text),
            _ => None,
        });
        self.sheet.raw_input_hook(ctx, raw);
        // Files dropped on the window open as workbooks.
        for f in std::mem::take(&mut raw.dropped_files) {
            if let Some(p) = f.path().to_str().map(str::to_string).filter(|p| !p.is_empty()) {
                self.open(&p);
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.sheet.ui(ui);
        // Android: text egui copies (text boxes, a failed HTML copy) goes to the system clipboard.
        #[cfg(target_os = "android")]
        for text in clipboard::take_copied_text(ui.ctx()) {
            if let Err(e) = craft_android::android::clipboard::set(craft_android::android::clipboard::Clip::Text(text)) {
                log::warn!("dexgridcraft: clipboard: {e}");
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_prefs();
    }
}

/// The window's options, shared by Android and the desktop binary.
pub fn native_options(renderer: eframe::Renderer) -> eframe::NativeOptions {
    eframe::NativeOptions {
        renderer,
        viewport: egui::ViewportBuilder::default()
            .with_title("DexGridCraft")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([640.0, 420.0])
            .with_app_id("io.github.dexgridcraft")
            .with_drag_and_drop(true),
        ..Default::default()
    }
}

/// Run the window with wgpu, then with glow if wgpu can't start (old or unusual GPU drivers).
/// `create` builds the app once a renderer has started.
pub fn run(
    mut options: impl FnMut(eframe::Renderer) -> eframe::NativeOptions,
    mut create: impl FnMut(&eframe::CreationContext<'_>) -> Box<dyn eframe::App>,
) {
    let mut last_error = None;
    for renderer in [eframe::Renderer::Wgpu, eframe::Renderer::Glow] {
        let result = eframe::run_native("DexGridCraft", options(renderer), Box::new(|cc| Ok(create(cc))));
        match result {
            Ok(()) => return,
            Err(e) => {
                log::error!("dexgridcraft: the {renderer} renderer could not start: {e}");
                #[cfg(target_os = "android")]
                craft_android::android::diag(&format!("the {renderer} renderer could not start: {e}"));
                last_error = Some(e);
            }
        }
    }
    if let Some(e) = last_error {
        log::error!("dexgridcraft: no renderer could start: {e}");
    }
}

/// The GridCraft release `upstream/` is at (its workspace version).
pub fn gridcraft_version() -> &'static str {
    const MANIFEST: &str = include_str!("../../../upstream/Cargo.toml");
    MANIFEST.lines().find_map(|l| l.strip_prefix("version = \"").and_then(|v| v.strip_suffix('"'))).unwrap_or("unknown")
}

/// The renderer that started, for the diagnostics report.
pub fn renderer_line(cc: &eframe::CreationContext<'_>) -> String {
    match &cc.wgpu_render_state {
        Some(state) => {
            let adapter = state.adapter.get_info();
            format!("renderer wgpu, {} ({:?}, {:?})", adapter.name, adapter.device_type, adapter.backend)
        }
        None if cc.gl.is_some() => "renderer glow (OpenGL ES)".to_owned(),
        None => "renderer unknown".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upstream_has_a_version() {
        assert!(gridcraft_version().split('.').count() == 3, "{}", gridcraft_version());
    }

    #[test]
    fn the_suggested_name_gives_the_format() {
        assert_eq!(format_of("Book1.xlsx"), "xlsx");
        assert_eq!(format_of("Book1.PDF"), "pdf");
        assert_eq!(format_of("Report.csv"), "csv");
        assert_eq!(format_of("Page.html"), "html");
        assert_eq!(format_of("Copy"), "xlsx");
        assert_eq!(format_of("Old.xlsm"), "xlsx");
    }

    #[test]
    fn each_format_saves_with_its_command() {
        assert_eq!(save_command("/d/a.xlsx", "xlsx"), ("file.save", json!({"path": "/d/a.xlsx"})));
        assert_eq!(save_command("/d/a", "xlsx"), ("file.save", json!({"path": "/d/a.xlsx"})));
        assert_eq!(save_command("/d/a.pdf", "pdf"), ("file.exportPdf", json!({"path": "/d/a.pdf"})));
        assert_eq!(save_command("/d/a.csv", "csv"), ("file.exportCsv", json!({"path": "/d/a.csv"})));
        assert_eq!(save_command("/d/a.html", "html"), ("file.exportHtml", json!({"path": "/d/a.html"})));
    }
}
