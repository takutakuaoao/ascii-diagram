// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use unicode_width::UnicodeWidthChar;

#[allow(dead_code)]
fn rectangle_frame(text: &str) -> String {
    if text.is_empty() {
        return empty_rectangle_frame();
    }
    rectangle_frame_with_text(text)
}

fn empty_rectangle_frame() -> String {
    [
        "┌ ─ ┐",
        "└ ─ ┘",
    ]
    .join("\n")
}

fn rectangle_frame_with_text(text: &str) -> String {
    [
        format!("┌ {} ┐", outputs_horizontal_border(text)),
        format!("│ {} │", text),
        format!("└ {} ┘", outputs_horizontal_border(text)),
    ]
    .join("\n")
}

fn outputs_horizontal_border(text: &str) -> String {
    text.chars()
        .map(|c| if is_fullwidth(c) { 'ー' } else { '-' })
        .collect()
}

fn is_fullwidth(c: char) -> bool {
    c.width_cjk().unwrap_or(1) == 2
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_frame_displays_empty_rectangle_when_text_is_empty() {
        let expected = [
            "┌ ─ ┐",
            "└ ─ ┘",
        ]
        .join("\n");
        assert_eq!(rectangle_frame(""), expected);
    }

    #[test]
    fn rectangle_frame_displays_rectangle_with_hyphens_matching_fullwidth_halfwidth_when_text_is_present() {
        let expected = [
            "┌ ー-ー-ー-ー-ー- ┐",
            "│ あaいbうcえeおo │",
            "└ ー-ー-ー-ー-ー- ┘",
        ]
        .join("\n");
        assert_eq!(rectangle_frame("あaいbうcえeおo"), expected);
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