use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};
use std::{error::Error, io};
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

    fn set_input_string(&mut self, s: &str) {
        self.input_string = s.to_string();
    }

    fn get_string_stats(&self) -> StringStats {
        StringStats {
            len: self.input_string.len(),
            capacity: self.input_string.capacity(),
            pointer: self.input_string.as_ptr() as usize,
        }
    }

    fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_quit = true;
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

fn ui(f: &mut Frame, _app: &mut App) {
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

    let menu = Block::default()
        .title("Main Menu")
        .borders(Borders::ALL);
    f.render_widget(menu, main_chunks[0]);

    let content = Block::default()
        .title("Content")
        .borders(Borders::ALL);
    f.render_widget(content, main_chunks[1]);

    let visualizer_text = render_memory_visualization(_app);
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
    fn test_memory_visualization_output() {
        let app = App::new();
        let stats = app.get_string_stats();
        let visual = render_memory_visualization(&app);
        
        // Should contain Stack and Heap sections
        assert!(visual.contains("STACK"));
        assert!(visual.contains("HEAP"));
        
        // Should contain pointer, capacity, and length values
        assert!(visual.contains(&stats.len.to_string()));
        assert!(visual.contains(&stats.capacity.to_string()));
        assert!(visual.contains(&format!("0x{:x}", stats.pointer)));
    }
}

