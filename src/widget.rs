use ratatui::widgets::{Block, Paragraph};
pub fn create_widgets() -> [Paragraph<'static>; 2] {
    // Construct a `Paragraph` widget surrounded by a block.
    let text = Paragraph::new("Rvim text").block(Block::default());

    let status_bar = Paragraph::new("Rvim status");

    [text, status_bar]
}
