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
    let branch = match(repo.head()){
        Ok(head) => match(head.shorthand()){
            Some(name) => String::from(name),
            None => String::from(""),
        },
        Err(_) => String::from("")
    };

    String::from(branch)
}

fn main() {

    let repo = Repository::discover(".").unwrap();

    println!("{:?}", get_branch_name(&repo) )

    //println!("{} {} {} {}{}", get_host_name(), get_time(), get_repo(), get_current_dir(), get_newline());
}
