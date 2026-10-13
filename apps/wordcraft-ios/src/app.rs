//! The app eframe runs on iPadOS: WordCraft's UI plus what the host owns (safe area, saving the
//! preferences when the app leaves the foreground).
use std::path::PathBuf;

use eframe::egui;
use wordcraft_ui_egui::WordApp;

pub struct IosApp {
    app: WordApp,
    /// The app's support folder, where the preferences live (none: nowhere to keep them).
    root: Option<PathBuf>,
    was_focused: bool,
    last_safe_area: Option<String>,
    /// Takes the documents the app saves to where the user keeps them.
    sync: Option<crate::documents::DocumentSync>,
}

impl IosApp {
    pub fn new(app: WordApp, root: Option<PathBuf>) -> Self {
        let sync = root.clone().map(crate::documents::DocumentSync::new);
        Self { app, root, was_focused: true, last_safe_area: None, sync }
    }

    fn save_prefs(&self) {
        if let Some(root) = &self.root
            && let Err(error) = crate::prefs::save(&self.app, root)
        {
            log::warn!("Could not save the preferences: {error}");
        }
    }
}

impl eframe::App for IosApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::safe_area::report(ctx, &mut self.last_safe_area);
        // iPadOS may end a backgrounded app without a word: keep the preferences current.
        let focused = ctx.input(|i| i.focused);
        if self.was_focused && !focused {
            self.save_prefs();
        }
        self.was_focused = focused;
        self.app.logic(ctx);
        if let Some(sync) = &mut self.sync {
            sync.poll(|message| self.app.status(message));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.app.ui(ui);
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.app.raw_input_hook(raw_input);
        crate::safe_area::apply(raw_input);
    }

    fn on_exit(&mut self) {
        self.save_prefs();
    }
}
