use git2::Repository;
use inquire::MultiSelect;

fn main() {
    let repo = open_curr_repo();

    let branches = get_branches(&repo);

    pp_branches(&branches);
}

fn open_curr_repo() -> Repository {
    match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

fn get_branches(repo: &Repository) -> Vec<(git2::Branch, git2::BranchType)> {
    match repo.branches(None) {
        Ok(branches) => branches.filter_map(|b| b.ok()).collect(),
        Err(e) => panic!("failed to get branches"),
    }
}

fn pp_branches(branches: &[(git2::Branch, git2::BranchType)]) {
    let items = branches
        .iter()
        .map(|b| b.0.name().unwrap().unwrap())
        .collect::<Vec<_>>();
    let ans = MultiSelect::new("Git branches:", items).prompt();

    match ans {
        Ok(_) => println!("I'll get right on it"),
        Err(_) => println!("The shopping list could not be processed"),
    }
    // match selection {
    //     Some(index) => println!("User selected item : {}", items[index]),
    //     None => println!("User did not select anything"),
    // }
    // for tuple in branches {
    //     println!("{}", tuple.0.name().unwrap().unwrap())
    // }
}
