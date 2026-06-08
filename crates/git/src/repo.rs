use std::path::Path;

use anyhow::Result;
use git2::Oid;
use git2::Patch;
use git2::Repository;
use types::{CommitHash, DiffLine, Enrichments, FileDiff, FileStatus,
            HunkRange, LineKind, Region};

pub struct GitRepo {
    repo: Repository
}

impl GitRepo {
    pub fn open(path: &str) -> Result<Self> {
        let repo = Repository::open(path)?;
        Ok(Self { repo })
    }

    pub fn diff(&self, base: &CommitHash, head: &CommitHash) -> Result<Vec<FileDiff>> {
        let base_oid = Oid::from_str(base.0.as_str())?;
        let head_oid = Oid::from_str(head.0.as_str())?;
        let base_commit = self.repo.find_commit(base_oid)?;
        let head_commit = self.repo.find_commit(head_oid)?;
        let diff = self.repo.diff_tree_to_tree(
            Some(&base_commit.tree()?),
            Some(&head_commit.tree()?),
            None,
        )?;

        let mut files = Vec::new();

        for idx in 0..diff.deltas().len() {
            if let Some(patch) = Patch::from_diff(&diff, idx)? {
                let delta = patch.delta();
                let mut regions = Vec::new();

                for hunk_idx in 0..patch.num_hunks() {
                    let (hunk, num_lines) = patch.hunk(hunk_idx)?;
                    let mut lines = Vec::new();

                    for line_idx in 0..num_lines {
                        let line = patch.line_in_hunk(hunk_idx, line_idx)?;
                        let kind = match line.origin() {
                            '+' => LineKind::Added,
                            '-' => LineKind::Removed,
                            ' ' => LineKind::Context,
                            _ => continue,
                        };
                        lines.push(DiffLine {
                            kind,
                            content: String::from_utf8_lossy(line.content()).to_string(),
                            old_no: line.old_lineno(),
                            new_no: line.new_lineno(),
                        });
                    }

                    regions.push(Region {
                        range: HunkRange {
                            old_start: hunk.old_start(),
                            old_lines: hunk.old_lines(),
                            new_start: hunk.new_start(),
                            new_lines: hunk.new_lines(),
                        },
                        lines,
                        enrichments: Enrichments::new(),
                    });
                }

                let status = match delta.status() {
                    git2::Delta::Added => FileStatus::Added,
                    git2::Delta::Deleted => FileStatus::Deleted,
                    git2::Delta::Modified => FileStatus::Modified,
                    git2::Delta::Renamed => FileStatus::Renamed,
                    _ => FileStatus::Modified,
                };

                files.push(FileDiff {
                    old_path: delta.old_file().path().map(|p| p.to_path_buf()),
                    new_path: delta.new_file().path().map(|p| p.to_path_buf()),
                    status,
                    regions,
                    enrichments: Enrichments::new(),
                });
            }
        }

        Ok(files)
    }

    pub fn resolve(&self, refspec: &str) -> Result<CommitHash> {
        let obj = self.repo.revparse_single(refspec)?;
        Ok(CommitHash(obj.id().to_string()))
    }

    pub fn blame(&self, _path: &Path, _at: &CommitHash) -> Result<serde_json::Value> {
        todo!()
    }
}
