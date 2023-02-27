use tauri::{command, AppHandle, LogicalSize, Manager, Size};

#[command]
pub fn resize_window(height: f64, app_handle: AppHandle) -> String {
    let window = app_handle.get_window("main").unwrap();
    window.set_size(Size::Logical(LogicalSize {
        width: 675.0,
        height,
    }));
    return "testing".to_string();
}
