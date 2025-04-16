mod message;
mod tui;
mod provider;
mod prompt;
mod git;
mod notify;

use clap::{Parser, Subcommand};
use notify::Notify;
use provider::MessageProvider;
use git::get_staged_diff;
use tui::App;
// use ratatui::{TerminalOptions, Viewport};

#[derive(Parser)]
#[command(version)]
#[command(about = "Generating commit message over your changes", long_about=None)]
pub struct ArgumentParser {
    #[arg(short, long)]
    no_interactive: bool,
    #[arg(short, long)]
    preview: bool,
    #[command(subcommand)]
    commands: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    // operate configuring for tatl
    Config {
        // list current config
        #[arg(short, long)]
        list: bool
    }
}

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed color_eyre");
    let ntfy = Notify::default();
    let parser = ArgumentParser::parse();
    println!("{}", parser.no_interactive);
    match get_staged_diff(".") {
        Ok(diff_content) => {
            let terminal = ratatui::init();
            let provider = MessageProvider::new("gpt-4o");
            match App::new().run(terminal, provider, diff_content, 5).await {
                Ok(_) => (),
                Err(e) => {
                    ntfy.error(e.to_string());
                }
            }
            ratatui::restore();
        }
        Err(e) => {
            ntfy.error(e.to_string());
        }
    }
}
