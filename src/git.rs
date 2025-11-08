use git2::{self, BranchType, Repository};

pub struct Git {
    pub repo: Repository,
}

impl Git {
    pub fn new() -> Self {
        let repo = Repository::open("./").expect("failed to open repo");
        Self { repo }
    }

    pub fn get_branches(&mut self) {
        let _branches = match self.repo.branches(Some(BranchType::Local)) {
            Ok(branches) => branches,
            Err(e) => panic!("failed to open: {}", e),
        };

        for branch_result in _branches {
            if let Ok((branch, kind)) = branch_result {
                // branch.name() -> Result<Option<&str>, _>
                let name = branch.name().ok().flatten().unwrap_or("<nom invalide>");
                println!("{name} ({kind:?})");
            }
        }
    }
}
