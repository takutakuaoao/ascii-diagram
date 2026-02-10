// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use unicode_width::UnicodeWidthChar;

const FRAME_CORNER: char = '+';

#[allow(dead_code)]
struct Rectangle {
    content: Content,
}

#[allow(dead_code)]
impl Rectangle {
    fn new_from_text(text: &str) -> Self {
        Rectangle { content: Content::new_from_text(text) }
    }

    fn horizontal_border(&self) -> String {
        self.content
            .longest_line()
            .remap_by_width('-', 'ー')
            .to_string()
    }

    fn outputs_border(&self) -> String {
        format!(
            "{} {} {}",
            FRAME_CORNER,
            self.horizontal_border(),
            FRAME_CORNER
        )
    }

    fn outputs(&self) -> String {
        [
            self.outputs_border(),
            self.content.outputs(),
            self.outputs_border(),
        ]
        .join("\n")    
    }
}

struct Content {
    lines: Vec<Line>,
}

impl Content {
    fn new_from_text(text: &str) -> Self {
        let lines: Vec<Line> = text
            .split('\n')
            .map(|s| Line::new(s.to_string()))
            .collect();
        Content { lines }
    }

    fn longest_line(&self) -> Line {
        self.lines
            .iter()
            .max_by_key(|line| line.count_chars())
            .cloned()
            .unwrap_or_default()
    }

    fn outputs(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.wrap_with('│').to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Line {
    text: String,
}

impl Line {
    fn new(text: String) -> Self {
        Line { text }
    }

    fn count_chars(&self) -> usize {
        self.text.chars().count()
    }

    fn remap_by_width(&self, half: char, full: char) -> Self {
        let result: String = self
            .text
            .chars()
            .map(|c| if is_fullwidth(c) { full } else { half })
            .collect();
        Line::new(result)
    }

    fn wrap_with(&self, symbol: char) -> Self {
        let formatted = format!("{} {} {}", symbol, self.text, symbol);

        Line::new(formatted)
    }

    fn to_string(&self) -> String {
        self.text.to_string()
    }
}

#[allow(dead_code)]
fn rectangle_frame(text: &str) -> String {
    if text.is_empty() {
        return empty_rectangle_frame();
    }
    rectangle_frame_with_text(text)
}

fn empty_rectangle_frame() -> String {
    [
        format!("{} ─ {}", FRAME_CORNER, FRAME_CORNER),
        format!("{} ─ {}", FRAME_CORNER, FRAME_CORNER),
    ]
    .join("\n")
}

fn rectangle_frame_with_text(text: &str) -> String {
    let rectangle = Rectangle::new_from_text(text);

    rectangle.outputs()
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
            "+ ─ +",
            "+ ─ +",
        ]
        .join("\n");
        assert_eq!(rectangle_frame(""), expected);
    }

    #[test]
    fn rectangle_frame_displays_rectangle_with_hyphens_matching_fullwidth_halfwidth_when_text_is_present() {
        let expected = [
            "+ ー-ー-ー-ー-ー- +",
            "│ あaいbうcえeおo │",
            "+ ー-ー-ー-ー-ー- +",
        ]
        .join("\n");
        assert_eq!(rectangle_frame("あaいbうcえeおo"), expected);
    }

    #[test]
    fn rectangle_frame_displays_multiple_lines_when_text_contains_newline() {
        let expected = [
            "+ ーーーーー +",
            "│ あいうえお │",
            "│ あいうえお │",
            "+ ーーーーー +",
        ]
        .join("\n");

        assert_eq!(rectangle_frame("あいうえお\nあいうえお"), expected);
    }

    #[test]
    fn line_remap() {
        let line = Line::new(String::from("aあ"));
        let actual = line.remap_by_width('-', 'ー');

        assert_eq!(Line::new(String::from("-ー")), actual);
    }

    #[test]
    fn line_wrap_with() {
        let line = Line::new(String::from("ab"));
        let actual = line.wrap_with('|');

        assert_eq!(actual, Line::new(String::from("| ab |")));
    }

    #[test]
    fn content_longest_line() {
        let content = Content::new_from_text("abc\nabcd");
        let longest = content.longest_line();

        assert_eq!(longest, Line::new(String::from("abcd")));
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