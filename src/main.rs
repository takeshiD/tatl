mod message;
mod tui;
mod provider;
mod prompt;
mod git;

use provider::MessageProvider;
use git::get_staged_diff;
use tui::App;
use color_eyre::Result;
// use ratatui::{TerminalOptions, Viewport};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    match get_staged_diff(".") {
        Ok(diff_content) => {
            let terminal = ratatui::init();
            let provider = MessageProvider::new("gpt-4o");
            let result = App::new().run(terminal, provider, diff_content, 5).await;
            ratatui::restore();
            result
        }
        Err(e) => {
            Err(e)
        }
    }
}
