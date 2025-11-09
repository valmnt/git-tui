use color_eyre::eyre::Context;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
};

mod git;

fn main() {
    let app = App::new();
    let terminal = ratatui::init();
    run(terminal, app);
}

struct App {
    tree_lines: Vec<String>,
    vertical_scroll: usize,
    horizontal_scroll: usize,
}

impl App {
    fn new() -> Self {
        let mut git = git::Git::new();
        let tree_lines = git.build_tree_lines();
        Self {
            tree_lines,
            vertical_scroll: 0,
            horizontal_scroll: 0,
        }
    }

    fn scroll_up(&mut self, speed: usize) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(speed)
    }

    fn scroll_down(&mut self, speed: usize) {
        let max_scroll = self.tree_lines.len().saturating_sub(speed);
        if self.vertical_scroll < max_scroll {
            self.vertical_scroll += speed;
        }
    }

    fn scroll_offsets(&self) -> (u16, u16) {
        (
            self.vertical_scroll.min(u16::MAX as usize) as u16,
            self.horizontal_scroll.min(u16::MAX as usize) as u16,
        )
    }
}

fn run(mut terminal: DefaultTerminal, mut app: App) {
    loop {
        let _ = terminal.draw(|frame| draw(frame, &app));
        handle_events(&mut app);
        if exit() {
            break;
        }
    }
}

fn draw(frame: &mut Frame, app: &App) {
    let content = if app.tree_lines.is_empty() {
        "No git data available".to_string()
    } else {
        app.tree_lines.join("\n")
    };

    let paragraph = Paragraph::new(content).scroll(app.scroll_offsets());
    frame.render_widget(paragraph, frame.area());
}

fn handle_events(app: &mut App) -> bool {
    if let Ok(event) = event::read().context("event read failed") {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('h') => {
                    app.scroll_up(15);
                }
                KeyCode::Char('j') => {
                    app.scroll_down(15);
                }
                KeyCode::Char('k') => {
                    app.scroll_up(5);
                }
                KeyCode::Char('l') => {
                    app.scroll_down(5);
                }
                _ => {}
            }
        }
    }

    false
}

fn exit() -> bool {
    if let Ok(Event::Key(key)) = event::read().context("event read failed") {
        return KeyCode::Char('q') == key.code;
    }

    false
}
