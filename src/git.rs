use git2::Repository;

pub fn get_repo() {
    let _repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    println!("Opened repository at: {:?}", _repo.path());
}
