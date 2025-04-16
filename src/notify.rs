use owo_colors::{OwoColorize, Style};

pub struct Notify {
    error: Style,
    warning: Style,
    info: Style,
    debug: Style,
}

impl Default for Notify {
    fn default() -> Self {
        Self {
            error: Style::new().red(),
            warning: Style::new().yellow(),
            info: Style::new().blue(),
            debug: Style::new().purple(),
        }
    }
}

impl Notify {
    pub fn error(self, msg: String) {
        eprintln!("{} {}", "ERROR!".style(self.error), msg);
    }
    pub fn warning(self, msg: String) {
        eprintln!("{} {}", "WARNING!".style(self.warning), msg);
    }
    pub fn info(self, msg: String) {
        println!("{} {}", "INFO!".style(self.info), msg);
    }
    pub fn debug(self, msg: String) {
        println!("{} {}", "DEBUG".style(self.debug), msg);
    }
}
