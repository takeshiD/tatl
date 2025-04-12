use std::fmt;

#[derive(Clone)]
pub enum ConventionalType {
    Build,
    CI,
    Chore,
    Docs,
    Feat,
    Fix,
    Perf,
    Refactor,
    Revert,
    Style,
    Test,
}

#[derive(Clone)]
pub struct CommitMessage {
    pub msgtype: Option<ConventionalType>,
    pub breaking_change: bool,
    pub scope: Option<String>,
    pub subject: String,
    pub body: Option<String>,
    pub select: bool,
}

impl CommitMessage {
    pub fn new(
        msgtype: Option<ConventionalType>,
        breaking_change: bool,
        scope: Option<String>,
        subject: String,
        body: Option<String>,
        select: bool,
    ) -> Self {
        Self {
            msgtype,
            breaking_change,
            scope,
            subject,
            body,
            select,
        }
    }
}

impl fmt::Display for ConventionalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        let value = match self {
            Self::Build     => "build",
            Self::CI        => "ci",
            Self::Chore     => "chore",
            Self::Docs      => "docs",
            Self::Feat      => "feat",
            Self::Fix       => "fix",
            Self::Perf      => "perf",
            Self::Refactor  => "refactor",
            Self::Revert    => "revert",
            Self::Style     => "style",
            Self::Test      => "test",
        };
        write!(f, "{}", value)
    }
}

impl fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.subject)
    }
}
