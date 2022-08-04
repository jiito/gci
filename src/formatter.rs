use git2::{Branch, Time};

struct CheckoutBranch {
    name: String,
    last_commit: String,
    time_commited: Time,
}

fn transform_branches(branches: &Vec<&Branch>) {
    let res = branches.into_iter().map(|b| {
        let name = b.name().unwrap();
        let commit = b.into_reference().peel_to_commit().unwrap();
        println!("{commit:?}")
    });
}
