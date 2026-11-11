//! The Android side: the entry point and GridCraft's [`Services`] on craft-android (the Storage
//! Access Framework's pickers, the clipboard, printing, links). Everything else (the JNI bridge,
//! the environment, the activity's input, the lifecycle) is craft-android's.

use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use craft_android::android::{self as platform, AndroidApp, clipboard::Clip, pickers::PickKind};
use gridcraft_ui_egui::printing::{Discover, Printer, Submit};
use gridcraft_ui_egui::{Inbox, Services};

use crate::{Answers, GridApp, Launch, Picked};

/// The one "printer" GridCraft's Print page lists: Android's print dialog picks the real one.
const ANDROID_PRINTER: &str = "Android";

/// Entry point, called by android-activity on the activity's own thread.
#[allow(unsafe_code)] // `no_mangle` is how android-activity's glue finds the entry point
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    let config = platform::Config { log_tag: "dexgridcraft", asset_dirs: Vec::new() };
    let Some(started) = platform::start(&app, &config) else { return };
    platform::diag(&format!("DexGridCraft {} (GridCraft {})", env!("CARGO_PKG_VERSION"), crate::gridcraft_version()));
    let print_dir = Arc::new(started.data_dir.join("cache/print"));
    let launch = Launch::default();
    crate::run(
        |renderer| platform::native_options(&app, crate::native_options(renderer)),
        |cc| {
            platform::events::set_context(cc.egui_ctx.clone());
            platform::diag(&crate::renderer_line(cc));
            let print_dir = print_dir.clone();
            let grid = GridApp::new(cc, &launch, move |answers| services(answers, print_dir));
            // craft-android's shell: the activity's mouse, keyboard and insets, Back as the close
            // button (GridCraft asks about unsaved workbooks), the title for DeX's taskbar.
            Box::new(craft_android::Shell::new(grid).on_focus_lost(|a| a.save_prefs()).title(GridApp::title))
        },
    );
    // The window closed (GridCraft allowed it, after asking about unsaved changes).
    platform::finish();
}

/// What GridCraft opens, for the Open picker (as the desktop's File › Open filter).
const SPREADSHEETS: [&str; 10] = ["xlsx", "xlsm", "xls", "xlsb", "ods", "csv", "tsv", "txt", "slk", "json"];

fn filters(name: &str, extensions: &[&str]) -> Vec<(String, Vec<String>)> {
    vec![(name.to_owned(), extensions.iter().map(|e| (*e).to_owned()).collect())]
}

/// Wait for a picker's answer on a worker thread, then hand it to `then`. The picker itself must
/// be started on the interface thread, which can't wait: the activity's pause waits for it.
fn when_picked(picked: platform::pickers::Picked, then: impl FnOnce(Vec<PathBuf>) + Send + 'static) {
    let spawned = std::thread::Builder::new().name("picker".into()).spawn(move || then(block_on(picked)));
    if let Err(e) = spawned {
        log::error!("dexgridcraft: picker thread: {e}");
    }
}

/// craft-android's picker futures block inside `poll` until the answer comes, so one poll is
/// the whole wait.
fn block_on<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        if let std::task::Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
        std::thread::yield_now();
    }
}

fn path_string(path: &std::path::Path) -> Option<String> {
    path.to_str().map(str::to_owned).filter(|p| !p.is_empty())
}

fn services(answers: Answers, print_dir: Arc<PathBuf>) -> Services {
    Services {
        // File › Open: the system picker; the workbook opens when it answers.
        open_async: Some(Box::new(move || {
            let answers = answers.clone();
            let picked = platform::pickers::pick(PickKind::OpenFile, None, &filters("Spreadsheets", &SPREADSHEETS), None);
            when_picked(picked, move |paths| {
                if let Some(path) = paths.first().and_then(|p| path_string(p)) {
                    answers.send(Picked(path));
                }
            });
        })),
        // Save As, the exports and Save a Copy: the system's Save picker; GridCraft runs what
        // follows (save, export) when it answers. A cancelled picker drops the reply.
        pick_save_async: Some(Box::new(|suggested: &str, reply: std::sync::mpsc::Sender<String>| {
            let format = crate::format_of(suggested);
            let name = std::path::Path::new(suggested).file_name().and_then(std::ffi::OsStr::to_str).unwrap_or("Book1.xlsx").to_owned();
            let picked = platform::pickers::pick(PickKind::Save, None, &filters(&format.to_uppercase(), &[format.as_str()]), Some(&name));
            when_picked(picked, move |paths| {
                if let Some(path) = paths.first().and_then(|p| path_string(p)) {
                    let _ = reply.send(path);
                    platform::events::wake();
                }
            });
        })),
        // Insert › Pictures: the picture's bytes, as GridCraft's web version reads them.
        pick_picture_async: Some(Box::new(|inbox: Inbox| {
            let picked = platform::pickers::pick(PickKind::OpenFile, None, &filters("Pictures (PNG, JPEG)", &["png", "jpg", "jpeg"]), None);
            when_picked(picked, move |paths| {
                let Some(path) = paths.first() else { return };
                match std::fs::read(path) {
                    Ok(bytes) => {
                        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "picture.png".into());
                        inbox.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push((name, bytes));
                        platform::events::wake();
                    }
                    Err(e) => log::warn!("dexgridcraft: picture {}: {e}", path.display()),
                }
            });
        })),
        copy_html: Some(Box::new(|html: &str, text: &str| platform::clipboard::set(Clip::Html { html: html.to_owned(), text: text.to_owned() }))),
        paste_html: Some(Box::new(|| match platform::clipboard::get()? {
            Clip::Html { html, .. } => Some(html),
            _ => None,
        })),
        paste_text: Some(Box::new(|| match platform::clipboard::get()? {
            Clip::Text(text) | Clip::Html { text, .. } => Some(text),
            _ => None,
        })),
        paste_image: Some(Box::new(|| match platform::clipboard::get()? {
            Clip::Image { width, height, rgba } => image::RgbaImage::from_raw(u32::try_from(width).ok()?, u32::try_from(height).ok()?, rgba),
            _ => None,
        })),
        // Links: an Intent (craft-android only lets http, https and mailto through).
        open_url: Some(Box::new(|url: &str| platform::desktop::open_url(url))),
        printers: Some(discover()),
        print_native: Some(submit(print_dir)),
        ..Default::default()
    }
}

fn discover() -> Discover {
    Box::new(|| {
        let (sender, receiver) = std::sync::mpsc::channel();
        let _ = sender.send(vec![Printer {
            name: ANDROID_PRINTER.to_owned(),
            label: "Android printing (choose the printer next)".to_owned(),
            default: true,
        }]);
        receiver
    })
}

/// GridCraft composes the print-ready PDF; Android's print dialog takes it from there (printer,
/// copies, two-sided).
fn submit(dir: Arc<PathBuf>) -> Submit {
    Box::new(move |bytes, settings| {
        let (sender, receiver) = std::sync::mpsc::channel();
        platform::diag(&format!("print: {} copies asked (the system dialog sets them)", settings.copies));
        let _ = sender.send(platform::print::print_pdf(&bytes, "GridCraft", &dir));
        receiver
    })
}
