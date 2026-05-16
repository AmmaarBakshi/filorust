pub mod app;

use std::path::PathBuf;
use app::AppState;
use slint::{ComponentHandle, ModelRc, VecModel};

slint::include_modules!();

fn update_ui_view(ui: &AppWindow, state: &AppState) {
    ui.set_current_path(state.current_directory.to_string_lossy().to_string().into());

    // Map files
    let gui_items: Vec<GuiFileItem> = state.items.iter().map(|item| {
        let size_str = if item.is_dir {
            "".to_string()
        } else if item.size > 1024 * 1024 {
            format!("{:.2} MB", (item.size as f64) / (1024.0 * 1024.0))
        } else {
            format!("{:.1} KB", (item.size as f64) / 1024.0)
        };

        GuiFileItem {
            name: item.name.clone().into(),
            is_dir: item.is_dir,
            size_str: size_str.into(),
            modified_str: item.modified.clone().unwrap_or_default().into(),
        }
    }).collect();

    ui.set_files(ModelRc::new(VecModel::from(gui_items)));

    // --- ADD THIS: Read disks from system and push to Slint ---
    let drive_strings: Vec<slint::SharedString> = filorust_core::get_system_drives()
        .into_iter()
        .map(|d| d.into())
        .collect();
    ui.set_drives(ModelRc::new(VecModel::from(drive_strings)));
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let state = AppState::new(PathBuf::from("."));

    update_ui_view(&ui, &state);

    let ui_handle = ui.as_weak();

    let state_rc = std::sync::Arc::new(std::sync::Mutex::new(state));

    let state_ctx = state_rc.clone();
    let ui_ctx = ui_handle.clone();
    ui.on_back_clicked(move || {
        let mut s = state_ctx.lock().unwrap();
        s.navigate_back();
        update_ui_view(&ui_ctx.unwrap(), &s);
    });

    let state_ctx = state_rc.clone();
    let ui_ctx = ui_handle.clone();
    ui.on_forward_clicked(move || {
        let mut s = state_ctx.lock().unwrap();
        s.navigate_forward();
        update_ui_view(&ui_ctx.unwrap(), &s);
    });

    let state_ctx = state_rc.clone();
    let ui_ctx = ui_handle.clone();
    ui.on_up_clicked(move || {
        let mut s = state_ctx.lock().unwrap();
        s.navigate_up();
        update_ui_view(&ui_ctx.unwrap(), &s);
    });

    let state_ctx = state_rc.clone();
    let ui_ctx = ui_handle.clone();
    ui.on_row_double_clicked(move |index| {
        let mut s = state_ctx.lock().unwrap();
        if let Some(item) = s.items.get(index as usize) {
            if item.is_dir {
                let target_path = item.path.clone();
                s.navigate_to(target_path);
                update_ui_view(&ui_ctx.unwrap(), &s);
            }
        }
    });

    let state_ctx = state_rc.clone();
    let ui_ctx = ui_handle.clone();
    ui.on_drive_clicked(move |drive_path| {
        let mut s = state_ctx.lock().unwrap();
        let target_path = PathBuf::from(drive_path.as_str());
        s.navigate_to(target_path);
        update_ui_view(&ui_ctx.unwrap(), &s);
    });
    ui.run()
}