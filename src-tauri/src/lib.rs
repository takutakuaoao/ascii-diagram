// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[allow(dead_code)]
fn rectangle_frame() -> String {
    return [
            "┌ ─ ┐",
            "└ ─ ┘",
        ]
        .join("\n");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_frame_returns_expected_shape() {
        let expected = [
            "┌ ─ ┐",
            "└ ─ ┘",
        ]
        .join("\n");
        assert_eq!(rectangle_frame(), expected);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}