//! The saved UI preferences (`ui.json` in the app's support folder), read before the first frame
//! and written when the app goes to the background or quits. The file is the one the desktop app
//! keeps, so a preferences file means the same thing on every platform.
use std::path::{Path, PathBuf};

use wordcraft_ui_egui::{UiState, WordApp};

/// Preferences are small; anything bigger is not one of ours.
const MAX_PREFS: u64 = 4 * 1024 * 1024;

pub fn path(root: &Path) -> PathBuf {
    root.join("ui.json")
}

/// The saved preferences, if there are readable ones.
pub fn read(root: &Path) -> Option<UiState> {
    UiState::from_json(&crate::storage::bounded_read(&path(root), MAX_PREFS).ok()?)
}

/// Put the saved preferences into `app`; recent files that live in a moved container are rebased.
pub fn load(app: &mut WordApp, saved: Option<UiState>, root: &Path) {
    if let Some(mut ui) = saved {
        crate::storage::remap_recent_files(&mut ui.recent, root);
        app.apply_prefs(ui);
    }
}

/// Write `app`'s preferences; a failed write keeps the previous file.
pub fn save(app: &WordApp, root: &Path) -> Result<(), String> {
    std::fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let bytes = serde_json::to_vec_pretty(&app.prefs()).map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_PREFS {
        return Err("preferences exceed their size limit".into());
    }
    crate::storage::write_atomic(&path(root), &bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wordcraft_engine::Session;

    fn temp(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("wordcraft-ios-prefs-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    fn app() -> WordApp {
        WordApp::new(Session::new(wordcraft_engine::blank_document(&wordcraft_engine::Region::default())), Default::default())
    }

    #[test]
    fn preferences_survive_a_relaunch() {
        let root = temp("relaunch");
        let mut first = app();
        first.ui.recent = vec!["/external/letter.docx".into()];
        save(&first, &root).unwrap();
        let mut again = app();
        load(&mut again, read(&root), &root);
        assert_eq!(again.ui.recent, ["/external/letter.docx"]);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn unreadable_preferences_are_ignored_not_fatal() {
        let root = temp("unreadable");
        std::fs::write(path(&root), b"{broken JSON").unwrap();
        assert!(read(&root).is_none());
        std::fs::write(path(&root), b"[]").unwrap();
        assert!(read(&root).is_none());
        std::fs::remove_dir_all(root).unwrap();
    }
}
