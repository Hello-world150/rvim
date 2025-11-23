use ratatui::Frame;
use ratatui::crossterm::event::{self, Event};

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
    let [text, status_bar] = rvim::widget::create_widgets();
    let [text_area, status_bar_area] = rvim::layout::create_area().areas(frame.area());

    frame.render_widget(text, text_area);
    frame.render_widget(status_bar, status_bar_area);
}
