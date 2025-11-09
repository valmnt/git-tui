use git2::{self, BranchType, Repository};

const DEFAULT_COMMIT_LIMIT: usize = 20;

pub struct CommitInfo {
    pub short_id: String,
    pub summary: String,
}

pub struct Git {
    pub repo: Repository,
}

impl Git {
    pub fn new() -> Self {
        let repo = Repository::open("../safeat_flutter").expect("failed to open repo");
        Self { repo }
    }

    pub fn get_branches(&mut self) -> Vec<String> {
        let _branches = match self.repo.branches(Some(BranchType::Local)) {
            Ok(branches) => branches,
            Err(e) => panic!("failed to open: {e}"),
        };

        let mut names: Vec<String> = Vec::new();

        for branch_result in _branches {
            if let Ok((branch, _kind)) = branch_result {
                let name = match branch.name() {
                    Ok(Some(name)) => name,
                    Ok(None) => "unknown",
                    Err(e) => panic!("unable to get the branch name: {e}"),
                }
                .to_string();
                names.push(name);
            }
        }

        return names;
    }

    pub fn get_commits(&mut self, branch_name: &str) -> Vec<CommitInfo> {
        let branch = format!("refs/heads/{}", branch_name);
        let oid: git2::Oid = match self.repo.revparse_single(&branch) {
            Ok(obj) => obj.id(),
            Err(e) => panic!("failed fetch oid: {e}"),
        };

        let mut walk = match self.repo.revwalk() {
            Ok(walk) => walk,
            Err(e) => panic!("failed to create walk: {e}"),
        };

        let _ = walk.push(oid);
        let _ = walk.set_sorting(git2::Sort::TIME);

        let mut commits: Vec<CommitInfo> = Vec::new();

        for result in walk {
            if commits.len() >= DEFAULT_COMMIT_LIMIT {
                break;
            }
            if let Ok(oid) = result {
                let commit = match self.repo.find_commit(oid) {
                    Ok(commit) => commit,
                    Err(e) => panic!("failed to find a commit: {e}"),
                };

                let mut short_id = commit.id().to_string();
                short_id.truncate(7);

                let summary = commit
                    .summary()
                    .unwrap_or("no message")
                    .lines()
                    .next()
                    .unwrap_or("no message")
                    .trim()
                    .to_string();

                commits.push(CommitInfo { short_id, summary });
            }
        }

        return commits;
    }

    pub fn build_tree_lines(&mut self) -> Vec<String> {
        let branches = self.get_branches();

        if branches.is_empty() {
            return vec!["No branches found".into()];
        }

        let mut lines = Vec::with_capacity(branches.len() * (DEFAULT_COMMIT_LIMIT + 1));
        lines.push("Git Tree".into());

        for branch in branches {
            let prefix = { "└─" };
            let line = format!("{prefix} {branch}");
            lines.push(line);

            let commits = self.get_commits(&branch);
            for (index, commit) in commits.iter().enumerate() {
                let prefix = "| *";
                let summary = &commit.summary;
                let hash = &commit.short_id;
                let line = format!("{prefix} {hash} {summary}");
                lines.push(line);

                let last_commit = index == commits.len() - 1;
                if last_commit {
                    let space = "| ".to_string();
                    lines.push(space);
                }
            }
        }

        return lines;
    }
}
