use std::fmt;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone)]
pub enum CommitType {
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

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone)]
pub struct CommitMessage {
    pub commit_type: Option<CommitType>,
    pub breaking_change: bool,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
}

impl CommitMessage {
    pub fn new(
        commit_type: Option<CommitType>,
        breaking_change: bool,
        scope: Option<String>,
        subject: String,
        body: Option<String>,
    ) -> Self {
        Self {
            commit_type,
            breaking_change,
            scope,
            description: subject,
            body,
        }
    }
}

impl fmt::Display for CommitType {
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
        write!(f, "{}", self.description)
    }
}
