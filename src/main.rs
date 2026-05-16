// /src/main.rs
pub mod app;

use std::path::PathBuf;
use app::AppState;
use slint::{ComponentHandle, ModelRc, VecModel};

// Include compiled Slint UI data
slint::include_modules!();

fn update_ui_view(ui: &AppWindow, state: &AppState) {
    // 1. Set the top address bar text
    ui.set_current_path(state.current_directory.to_string_lossy().to_string().into());

    // 2. Map core Rust file models into Slint GUI structures
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

    // 3. Shove the items array into Slint's active context window
    let model = VecModel::from(gui_items);
    ui.set_files(ModelRc::new(model));
}

fn main() -> Result<(), slint::PlatformError> {
    // Create UI window handle
    let ui = AppWindow::new()?;
    
    // Create engine pointing to the system profile home directory or current execution path
    let state = AppState::new(PathBuf::from("."));
    
    // Display original file list data structure
    update_ui_view(&ui, &state);

    // --- Wire Up Button Hooks ---
    let ui_handle = ui.as_weak();
    //let mut state_clone = AppState::new(PathBuf::from(".")); // Local thread copy
    
    // Track references safely with closures
    let state_rc = std::sync::Arc::new(std::sync::Mutex::new(state));

    // Hook Back Button
    let state_ctx = state_rc.clone();
    let ui_ctx = ui_handle.clone();
    ui.on_back_clicked(move || {
        let mut s = state_ctx.lock().unwrap();
        s.navigate_back();
        update_ui_view(&ui_ctx.unwrap(), &s);
    });

    // Hook Forward Button
    let state_ctx = state_rc.clone();
    let ui_ctx = ui_handle.clone();
    ui.on_forward_clicked(move || {
        let mut s = state_ctx.lock().unwrap();
        s.navigate_forward();
        update_ui_view(&ui_ctx.unwrap(), &s);
    });

    // Hook Parent Folder Up Button
    let state_ctx = state_rc.clone();
    let ui_ctx = ui_handle.clone();
    ui.on_up_clicked(move || {
        let mut s = state_ctx.lock().unwrap();
        s.navigate_up();
        update_ui_view(&ui_ctx.unwrap(), &s);
    });

    // Hook Item Row Double Click Actions (Enter into targeted folders)
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

    // Display Window Canvas Frame
    ui.run()
}