mod message;
mod tui;
mod provider;
mod prompt;
mod git;

use tui::App;
use provider::MessageProvider;
use color_eyre::Result;
use ratatui::{TerminalOptions, Viewport};

const DIFF_CONTENT: &str = "
diff --git a/src/main.rs b/src/main.rs
index 18f808a..d24f14b 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,17 +1,20 @@
 mod message;
 mod tui;
+mod provider;
 
-use anyhow::Result;
 use tui::App;
-
+use provider::MessageProvider;
+use color_eyre::Result;
 use ratatui::{TerminalOptions, Viewport};
 
 #[tokio::main]
 async fn main() -> Result<()> {
+    color_eyre::install();
     let terminal = ratatui::init_with_options(TerminalOptions {
         viewport: Viewport::Inline(10),
     });
-    let result = App::default().run(terminal);
+    let provider = MessageProvider::new(\"gpt-4o\", 3);
+    let result = App::new().run(terminal, provider).await;
     ratatui::restore();
     result
 }
";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(10),
    });
    let provider = MessageProvider::new("gpt-4o");
    let result = App::new().run(terminal, provider, DIFF_CONTENT, 3).await;
    ratatui::restore();
    result
}
