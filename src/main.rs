mod git;

fn main() {
    let mut git = git::Git::new();
    let branches = git.get_branches();

    for branch in branches {
        git.get_commits_branch(&branch);
    }
}
