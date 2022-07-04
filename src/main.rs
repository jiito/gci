use git2::Repository;

fn main() {
    println!("Hello, world!");
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e)
    };


    let branches = match repo.branches(None) {
        Ok(branches) => branches.filter_map(|x| x.ok()),
        Err(e) => panic!("failed to get branches")
    };

    for branch in branches {
        println!("{}", branch.0.name().unwrap().unwrap())
    }

}
