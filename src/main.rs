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
}

impl App {
    fn new() -> Self {
        let mut git = git::Git::new();
        let tree_lines = git.build_tree_lines();
        Self { tree_lines }
    }
}

fn run(mut terminal: DefaultTerminal, app: App) {
    loop {
        let _ = terminal.draw(|frame| draw(frame, &app));
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

    let paragraph = Paragraph::new(content);
    frame.render_widget(paragraph, frame.area());
}

fn exit() -> bool {
    if let Ok(Event::Key(key)) = event::read().context("event read failed") {
        return KeyCode::Char('q') == key.code;
    }

    return false;
}
