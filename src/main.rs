use ratatui::crossterm::event::{self, Event};
use ratatui::{
    Frame,
    widgets::{Block, Paragraph},
};

fn main() {
    let mut terminal = ratatui::init();
    // This is the main loop.
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    // Construct a `Paragraph` widget surrounded by a block with borders.
    let text = Paragraph::new("Rvim text").block(Block::default());
    let status_bar = Paragraph::new("Rvim status");

    let [text_area, status_bar_area] = create_area().areas(frame.area());

    frame.render_widget(text, text_area);
    frame.render_widget(status_bar, status_bar_area);
}

// Generate layout with two areas: text area and status bar area.
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::Layout;
fn create_area() -> Layout {
    // The first area is `text` area, the second is `status_bar` area.
    Layout::vertical([Fill(3), Length(1)])
}
