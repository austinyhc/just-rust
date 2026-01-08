use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
enum StringCategory {
    #[strum(to_string = "String & &str")]
    Standard,
    #[strum(to_string = "PathBuf & Path")]
    Path,
    #[strum(to_string = "OsString & OsStr")]
    OsString,
    #[strum(to_string = "CString & CStr")]
    CString,
    #[strum(to_string = "Cow<str>")]
    Cow,
    #[strum(to_string = "Vec<u8> & &[u8]")]
    Bytes,
}

struct StringStats {
    len: usize,
    capacity: usize,
    pointer: usize,
}

struct App {
    should_quit: bool,
    selected_category: StringCategory,
    input_string: String,
}

impl App {
    fn new() -> Self {
        Self {
            should_quit: false,
            selected_category: StringCategory::Standard,
            input_string: String::from("Hello Rust"),
        }
    }

    #[allow(dead_code)]
    fn set_input_string(&mut self, s: &str) {
        self.input_string = s.to_string();
    }

    fn get_string_stats(&self) -> StringStats {
        match self.selected_category {
            StringCategory::Standard => StringStats {
                len: self.input_string.len(),
                capacity: self.input_string.capacity(),
                pointer: self.input_string.as_ptr() as usize,
            },
            StringCategory::Path | StringCategory::OsString => {
                let os_str = std::ffi::OsStr::new(&self.input_string);
                // On Unix, this is just the bytes. 
                // Using as_encoded_bytes() which is stable since 1.74
                let bytes = os_str.as_encoded_bytes();
                StringStats {
                    len: bytes.len(),
                    capacity: 0, // OsStr doesn't expose capacity easily as it might be a slice
                    pointer: bytes.as_ptr() as usize,
                }
            }
            _ => StringStats {
                len: 0,
                capacity: 0,
                pointer: 0,
            },
        }
    }

    fn get_cpp_comparison(&self) -> &'static str {
        match self.selected_category {
            StringCategory::Standard => 
                "C++ Comparison:\n\
                 Rust: String        <-> C++: std::string (Owned, Heap-allocated)\n\
                 Rust: &str          <-> C++: std::string_view (Borrowed view)\n\
                 \n\
                 Key Difference: Rust Strings are always UTF-8. std::string is just bytes.",
            StringCategory::Path =>
                "C++ Comparison:\n\
                 Rust: PathBuf / Path <-> C++: std::filesystem::path\n\
                 \n\
                 Key Difference: Rust separates owned (PathBuf) and borrowed (Path) types strictly.",
            StringCategory::OsString =>
                "C++ Comparison:\n\
                 Rust: OsString / OsStr <-> C++: No direct equivalent (Platform-native strings)\n\
                 \n\
                 Key Difference: OsString handles platform-specific encodings (invalid UTF-8 on Linux, UTF-16 on Win).",
            _ => "Comparison not yet implemented."
        }
    }

    fn next_category(&mut self) {
        let categories: Vec<_> = StringCategory::iter().collect();
        let current_index = categories
            .iter()
            .position(|&c| c == self.selected_category)
            .unwrap_or(0);
        let next_index = (current_index + 1) % categories.len();
        self.selected_category = categories[next_index];
    }

    fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_quit = true;
        } else if c == 'n' || c == '\t' {
            self.next_category();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char(c) = key.code {
                app.on_key(c);
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(10),
        ])
        .split(f.area());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(chunks[0]);

    let categories: Vec<String> = StringCategory::iter()
        .map(|c| {
            if c == app.selected_category {
                format!("> {}", c)
            } else {
                format!("  {}", c)
            }
        })
        .collect();
    let menu_text = categories.join("\n");
    let menu = Paragraph::new(menu_text)
        .block(Block::default().title("Main Menu").borders(Borders::ALL));
    f.render_widget(menu, main_chunks[0]);

    let content_text = app.get_cpp_comparison();
    let content = Paragraph::new(content_text)
        .block(Block::default().title("Content").borders(Borders::ALL));
    f.render_widget(content, main_chunks[1]);

    let visualizer_text = render_memory_visualization(app);
    let visualizer = Paragraph::new(visualizer_text)
        .block(Block::default().title("Memory Visualizer").borders(Borders::ALL));
    f.render_widget(visualizer, chunks[1]);
}

fn render_memory_visualization(app: &App) -> String {
    let stats = app.get_string_stats();
    format!(
        "STACK\n\
         +----------+----------+----------+\n\
         |   ptr    |   cap    |   len    |\n\
         | 0x{:x} | {:^8} | {:^8} |\n\
         +----------+----------+----------+\n\
         \n\
         HEAP\n\
         +----------------------------------+\n\
         | {:^32} |\n\
         +----------------------------------+\n\
         Values: {:?}",
        stats.pointer, stats.capacity, stats.len, app.input_string, app.input_string.as_bytes()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_ui_layout_contains_main_sections() {
        let backend = TestBackend::new(100, 50);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();

        terminal.draw(|f| ui(f, &mut app)).unwrap();

        let buffer = terminal.backend().buffer();
        // Check for specific titles that indicate the layout
        let content = format!("{:?}", buffer);
        assert!(content.contains("Main Menu"), "UI missing 'Main Menu'");
        assert!(content.contains("Memory Visualizer"), "UI missing 'Memory Visualizer'");
        assert!(content.contains("Content"), "UI missing 'Content'");
    }

    #[test]
    fn test_category_switching() {
        let mut app = App::new();
        assert_eq!(app.selected_category, StringCategory::Standard);
        
        // Simulate 'Tab' or 'n' for next category (need to implement this)
        app.next_category();
        assert_eq!(app.selected_category, StringCategory::Path);
    }

    #[test]
    fn test_path_osstring_logic() {
        let mut app = App::new();
        app.selected_category = StringCategory::Path;
        
        // For Path, stats should reflect the internal OsString
        let stats = app.get_string_stats();
        assert!(stats.len > 0);
        
        app.selected_category = StringCategory::OsString;
        let stats_os = app.get_string_stats();
        assert_eq!(stats.len, stats_os.len);
    }
}

