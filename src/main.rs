use color_eyre::eyre::Context;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
};

mod git;

fn main() {
    process();
    let terminal = ratatui::init();
    run(terminal);
}

fn process() {
    let mut git = git::Git::new();
    let branches = git.get_branches();

    for branch in branches {
        git.get_commits_branch(&branch);
    }
}

fn run(mut terminal: DefaultTerminal) {
    loop {
        let _ = terminal.draw(draw);
        if exit() {
            break;
        }
    }
}

fn draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

fn exit() -> bool {
    if let Ok(Event::Key(key)) = event::read().context("event read failed") {
        return KeyCode::Char('q') == key.code;
    }

    return false;
}
