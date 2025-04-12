mod message;
mod tui;

use anyhow::Result;
use tui::App;

use ratatui::{TerminalOptions, Viewport};

#[tokio::main]
async fn main() -> Result<()> {
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(10),
    });
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}
