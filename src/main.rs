mod git;

fn main() {
    let mut git = git::Git::new();
    git.get_branches();
}
