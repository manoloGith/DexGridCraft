//! Workbook tabs under the ribbon (craft-android's [`TabStrip`]): on Android every workbook window
//! is shown in the activity's one window, so a tab per window switches between them, as DeX's
//! taskbar would between the desktop's native windows. GridCraft's own commands do the work: a
//! tab shows its window (`view.switchWindows`), × closes it (`window.close`, which asks about
//! unsaved changes), + starts a workbook (`file.new`).

use craft_android::tabs::{Tab, TabAction, TabStrip, TabStyle};
use gridcraft_ui_egui::SheetApp;
use serde_json::json;

/// One tab per shown window, in opening order: its title ("Book1", "Book1:2"), the dot while its
/// workbook has unsaved changes, the active one highlighted.
pub fn tabs_of(app: &SheetApp) -> Vec<Tab> {
    let s = &app.session;
    let active = s.active_window();
    s.windows()
        .iter()
        .filter(|w| !w.hidden)
        .map(|w| {
            let dirty = s.documents().iter().find(|d| d.uid == w.doc_uid).is_some_and(|d| d.is_dirty());
            Tab::new(w.id, s.window_title(w.id).unwrap_or_default(), dirty, Some(w.id) == active)
        })
        .collect()
}

/// The command a tab action runs (menu-style, so a close asks about unsaved changes).
pub fn command(action: TabAction) -> (&'static str, serde_json::Value) {
    match action {
        TabAction::Activate(window) => ("view.switchWindows", json!({ "window": window })),
        TabAction::Close(window) => ("window.close", json!({ "window": window })),
        TabAction::New => ("file.new", json!({})),
    }
}

/// The strip in GridCraft's colours (its theme leaves egui's faint background unset): the
/// ribbon's background behind the tabs, the shown one in the window's colour with the accent line.
pub fn style(ctx: &egui::Context, visuals: &egui::Visuals) -> TabStyle {
    let t = gridcraft_ui_egui::theme::Tokens::get(ctx);
    TabStyle {
        bar: t.ribbon,
        active: t.window,
        hover: t.hover,
        text: t.text,
        text_inactive: t.text_dim,
        accent: t.accent,
        separator: t.separator,
        ..TabStyle::from_visuals(visuals)
    }
}

/// GridCraft's `Services::below_ribbon` hook that draws the tabs.
#[allow(clippy::type_complexity)]
pub fn below_ribbon() -> Box<dyn Fn(&mut SheetApp, &mut egui::Ui)> {
    Box::new(|app: &mut SheetApp, ui: &mut egui::Ui| {
        let tabs = tabs_of(app);
        let lang = app.ui.language;
        let style = style(ui.ctx(), ui.visuals());
        let strip = TabStrip::new(&tabs).style(style).new_tooltip(lang.tr("New")).close_tooltip(lang.tr("Close"));
        if let Some(action) = strip.show_panel(ui).action {
            let (id, params) = command(action);
            app.run_or_alert(id, params);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gridcraft_engine::Session;
    use gridcraft_ui_egui::Services;

    fn app() -> SheetApp {
        let mut session = Session::new();
        session.new_workbook();
        SheetApp::new(session, Services::default())
    }

    #[test]
    fn each_window_gets_a_tab_and_the_active_one_is_marked() {
        let mut a = app();
        a.run("file.new", json!({})).unwrap_or_default();
        a.run("cell.set", json!({"cell": "A1", "input": "1"})).unwrap_or_default();
        let tabs = tabs_of(&a);
        assert_eq!(tabs.len(), 2);
        assert_eq!(tabs.iter().filter(|t| t.active).count(), 1);
        assert!(tabs[1].active && tabs[1].dirty, "{tabs:?}");
        assert!(!tabs[0].dirty);
        assert!(tabs.iter().all(|t| !t.title.is_empty()));
    }

    #[test]
    fn tab_actions_run_gridcrafts_commands() {
        let mut a = app();
        let first = a.session.active_window().unwrap_or_default();
        let (id, params) = command(TabAction::New);
        a.run_or_alert(id, params);
        assert_eq!(a.session.windows().len(), 2);
        let (id, params) = command(TabAction::Activate(first));
        a.run_or_alert(id, params);
        assert_eq!(a.session.active_window(), Some(first));
        let (id, params) = command(TabAction::Close(first));
        a.run_or_alert(id, params);
        assert_eq!(a.session.windows().len(), 1, "an unchanged workbook closes at once");
        assert_ne!(a.session.active_window(), Some(first));
    }

    #[test]
    fn closing_an_unsaved_workbook_asks_first() {
        let mut a = app();
        a.run("cell.set", json!({"cell": "A1", "input": "1"})).unwrap_or_default();
        let w = a.session.active_window().unwrap_or_default();
        let (id, params) = command(TabAction::Close(w));
        a.run_or_alert(id, params);
        assert_eq!(a.session.windows().len(), 1, "still open");
        assert_eq!(a.dialog.as_ref().map(|d| d.name()), Some("saveChanges"));
    }
}
