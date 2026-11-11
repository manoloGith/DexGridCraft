//! DexGridCraft in a desktop window, to iterate without a device:
//! `cargo run -p dexgridcraft --features desktop -- [book.xlsx…]`. No system pickers or clipboard
//! here (the Android build provides them); paths can be typed where GridCraft asks for one.

fn main() {
    let launch = dexgridcraft::Launch { files: std::env::args().skip(1).collect() };
    dexgridcraft::run(dexgridcraft::native_options, |cc| {
        log::info!("dexgridcraft: {}", dexgridcraft::renderer_line(cc));
        let app = dexgridcraft::GridApp::new(cc, &launch, |_answers| gridcraft_ui_egui::Services {
            below_ribbon: Some(dexgridcraft::tabs::below_ribbon()),
            ..Default::default()
        });
        Box::new(craft_android::Shell::new(app).on_focus_lost(|a| a.save_prefs()).title(dexgridcraft::GridApp::title))
    });
}
