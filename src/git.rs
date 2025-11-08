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
            Err(e) => panic!("failed to open: {}", e),
        };

        let mut names: Vec<String> = Vec::new();

        for branch_result in _branches {
            if let Ok((branch, _kind)) = branch_result {
                let name = branch.name().ok().flatten().unwrap_or("invalid");
                names.push(name.to_string());
            }
        }

        return names;
    }

    pub fn get_commits_branch(&mut self, branch: &str) {
        let reference = format!("refs/heads/{}", branch);
        let oid: git2::Oid = self.repo.revparse_single(&reference).unwrap().id();

        let mut walk = self.repo.revwalk().unwrap();
        let _ = walk.push(oid);
        let _ = walk.set_sorting(git2::Sort::TIME);

        for oid in walk {
            let commit = self.repo.find_commit(oid.unwrap());
            let c = commit.unwrap();
            let message = c.message().unwrap();
            println!("{message}");
        }
    }
}
