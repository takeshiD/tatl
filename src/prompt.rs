const PREAMBLE: &str = "Generate a Git commit message that conforms to the Conventional Commit from Git Diff Text.";

macro_rules! create_context {
($locale:expr) => {
    format!("
# Message Format
Allowed output formats are like following:

* <type>[optional scope]: <description>

* [optional body]

* [optional footer(s)]


'type' is Commit Type.

'optional scope': in addition to the commit type, captures the area within the codebase that the commit covers.

'description': Summary. This is usually the first line of a commit message.

'optional body': The body of the commit message. Generally, this is the third or subsequent line after the first line of the commit message, with a blank line between them. This is optional (according to common conventions).

'optional footer(s)': Footer. Generally, this is written at the end of the commit message, after a blank line. (Following common convention) Multiple entries are allowed, optional.
    * BREAKING CHANGE: <description>: Footer indicating a breaking change. Can also be expressed by adding an ! after the commit type (or scope, if there is one). Both can be specified.

# Commit Type
Commit Type are allowed like following:

* build
* chore
* ci
* docs
* feat
* fix
* perf
* refactor
* revert
* style
* test

# Message Language Locale
Please output a message in the natural language specified below.

Language: {}
", $locale)};
}

pub fn preamble_content() -> &'static str {
    PREAMBLE
}

pub fn context_content(locale: &str) -> String {
    create_context!(locale)
}
