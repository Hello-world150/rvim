// Generate layout with two areas: text area and status bar area.
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::Layout;
pub fn create_area() -> Layout {
    // The first area is `text` area, the second is `status_bar` area.
    Layout::vertical([Fill(3), Length(1)])
}
