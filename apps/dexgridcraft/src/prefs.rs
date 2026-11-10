//! GridCraft's preferences (`ui.json`, `prefs.json`), where its desktop binary keeps them on
//! Linux: `$XDG_CONFIG_HOME/gridcraft` (craft-android points XDG_CONFIG_HOME at private storage).

use gridcraft_ui_egui::SheetApp;

/// The settings folder, as the desktop binary's `config_dir` computes it on Linux.
pub fn config_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config")))
        .map(|c| c.join("gridcraft"))
}

/// As the desktop's `load_prefs`: the system's language and light or dark appearance until the
/// user chooses, then what they chose.
pub fn load(app: &mut SheetApp) {
    // Android: craft-android sets LANG from the activity's configuration (`es_ES.UTF-8`), the
    // language the user picked for apps; the system property sys-locale reads is the fallback.
    app.ui.language = std::env::var("LANG")
        .ok()
        .and_then(|lang| gridcraft_ui_egui::i18n::Language::parse(&lang))
        .unwrap_or_else(gridcraft_ui_egui::i18n::Language::system);
    app.ui.system_theme = true;
    let Some(dir) = config_dir() else { return };
    if let Ok(b) = std::fs::read(dir.join("ui.json"))
        && let Ok(ui) = serde_json::from_slice::<gridcraft_ui_egui::UiState>(&b)
    {
        let saved = serde_json::from_slice(&b).ok().and_then(|v| gridcraft_ui_egui::i18n::saved_language(&v));
        let language = saved.unwrap_or(app.ui.language);
        app.ui = ui.at_startup();
        app.ui.language = language;
    }
    if let Ok(b) = std::fs::read(dir.join("prefs.json"))
        && let Ok(p) = serde_json::from_slice(&b)
    {
        app.session.prefs = p;
    }
}

/// As the desktop's `save_prefs`. Android may end the process without asking, so it runs when
/// the window loses focus too.
pub fn save(app: &SheetApp) {
    let Some(dir) = config_dir() else { return };
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(b) = serde_json::to_vec_pretty(&app.ui) {
        let _ = std::fs::write(dir.join("ui.json"), b);
    }
    if let Ok(b) = serde_json::to_vec_pretty(&app.session.prefs) {
        let _ = std::fs::write(dir.join("prefs.json"), b);
    }
}
