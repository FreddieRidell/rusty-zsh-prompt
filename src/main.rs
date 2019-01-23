use git2::Repository;

fn get_host_name() -> String {
    String::from("%{%F{red}%}%n@%m")
}

fn get_current_dir() -> String {
    String::from("%{%F{green}%}%~")
}

fn get_time() -> String {
    String::from("%{%F{white}%}%D{%a %Y-%m-%d %T}")
}

fn get_repo() -> String {
    String::from("%{%F{white}%}(%{%F{magenta}%}repo%{%f%})")
}

fn get_newline() -> String {
    String::from("\n%f$ ")
}

fn get_branch_name(repo: &Repository) -> String {
    String::from(repo.head().unwrap().shorthand().unwrap())
}

fn get_statuses(repo: &Repository) -> () {
repo.statuses(None).unwrap().iter().for_each( |s| -> () {
println!("{:?}", s.status());
    });
}

fn main() {
    let repo = Repository::discover(".").unwrap();

    println!("{:?}", get_branch_name(&repo));

    get_statuses(&repo);

    //println!("{} {} {} {}{}", get_host_name(), get_time(), get_repo(), get_current_dir(), get_newline());
}
