use color_eyre::eyre::eyre;
use color_eyre::Result;
use git2::{DiffOptions, Repository};
use std::path::Path;

pub fn get_staged_diff(root_path: &str) -> Result<String> {
    let repo = Repository::open(Path::new(root_path))?;
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let index = repo.index()?;
    let mut diff_opts = DiffOptions::new();
    diff_opts.include_typechange(true);
    diff_opts.recurse_untracked_dirs(true);
    diff_opts.show_binary(true);

    let diff = repo.diff_tree_to_index(
        Some(&head_commit.tree()?),
        Some(&index),
        Some(&mut diff_opts),
    )?;

    let mut diff_output = String::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        match line.origin() {
            '+' | '-' | ' ' => {
                diff_output.push(line.origin());
                diff_output.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
                true
            }
            'H' | 'F' | 'B' => {
                diff_output.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
                true
            }
            _ => true,
        }
    })?;
    if diff_output.is_empty() {
        Err(eyre!("not found staged files"))
    } else {
        Ok(diff_output)
    }
}

// pub fn commit_message() {
//
// }
