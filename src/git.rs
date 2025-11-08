use git2::{self, BranchType, Repository};

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

    pub fn get_commits(&mut self, branch_name: &str) {
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

        for result in walk {
            if let Ok(oid) = result {
                let commit = match self.repo.find_commit(oid) {
                    Ok(commit) => commit,
                    Err(e) => panic!("failed to find a commit: {e}"),
                };

                let message = commit.message().unwrap_or("no message");
                println!("{message}");
            }
        }
    }
}
