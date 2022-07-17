use core::iter::FilterMap;
use git2::Branches;
use git2::Repository;

fn main() {
    let repo = open_curr_repo();

    let branches = get_branches(&repo);

    for tuple in branches {
        println!("{}", tuple.0.name().unwrap().unwrap())
    }
}

fn open_curr_repo() -> Repository {
    match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

fn get_branches<'a>(repo: &'a Repository) -> Vec<(git2::Branch<'a>, git2::BranchType)> {
    let branches = match repo.branches(None) {
        Ok(branches) => branches,
        Err(e) => panic!("failed to get branches"),
    };

    let branch_vec: Vec<_> = branches.filter_map(|b| b.ok()).collect();

    branch_vec
}
