use git2::BranchType;
use git2::Repository;
use git2::Status;
use std::env;

use std::collections::HashMap;

fn paint_text_in_color(color: &i8, text: String) -> String {
    format!("%F{{{}}}{}%f", color, text)
}

fn get_static_prompt() -> String {
    let prompt_color = env::var("PROMPT_COLOR").unwrap().parse::<i8>().unwrap();;

    vec![
        paint_text_in_color(&prompt_color, String::from("%n@%m")),
        paint_text_in_color(&7, String::from("%D{%a %Y-%m-%d %T}")),
        paint_text_in_color(&2, String::from("%~")),
        paint_text_in_color(&7, String::from("\n$ ")),
    ]
    .join(" ")
}

fn get_branch_name(repo: &Repository) -> String {
    paint_text_in_color(&5, String::from(repo.head().unwrap().shorthand().unwrap()))
}

fn get_statuses(repo: &Repository) -> String {
    let git_statuses_staged = vec![
        (Status::INDEX_DELETED, &1),
        (Status::INDEX_MODIFIED, &3),
        (Status::INDEX_NEW, &2),
        (Status::INDEX_RENAMED, &5),
        (Status::INDEX_TYPECHANGE, &4),
    ];
    let git_statuses_working = vec![
        (Status::CONFLICTED, &6),
        (Status::WT_DELETED, &1),
        (Status::WT_MODIFIED, &3),
        (Status::WT_NEW, &2),
        (Status::WT_RENAMED, &5),
        (Status::WT_TYPECHANGE, &4),
    ];

    let mut statuses_count = HashMap::new();
    repo.statuses(None).unwrap().iter().for_each(|s| -> () {
        let count = statuses_count.entry(s.status()).or_insert(0);
        *count += 1;
    });

    let staged_string = git_statuses_staged
        .iter()
        .map(|(status, color)| -> String {
            match statuses_count.get(status) {
                Some(n) => paint_text_in_color(color, format!("{}", n)),
                None => String::from(""),
            }
        })
        .filter(|s| s.len() > 0)
        .collect::<Vec<String>>()
        .join(" ");

    let working_string = git_statuses_working
        .iter()
        .map(|(status, color)| -> String {
            match statuses_count.get(status) {
                Some(n) => paint_text_in_color(color, format!("{}", n)),
                None => String::from(""),
            }
        })
        .filter(|s| s.len() > 0)
        .collect::<Vec<String>>()
        .join(" ");

    format!(" {}|{}", working_string, staged_string)
}

fn get_remote_diff(repo: &Repository) -> String {
    let branch_name = String::from(repo.head().unwrap().shorthand().unwrap());

    if let Ok(this_branch) = repo.find_branch(branch_name.as_str(), BranchType::Local) {

        let this_oid = repo.refname_to_id(this_branch.get().name().unwrap()).unwrap();
        if let Ok(upstream_branch) = this_branch.upstream() {
        let upstream_oid = repo.refname_to_id(upstream_branch.get().name().unwrap()).unwrap();


        if let Ok((ahead, behind)) = repo.graph_ahead_behind(this_oid, upstream_oid) {
            return paint_text_in_color(&4, format!("{}/{} ", ahead, behind));
        }
        }
    }

    return String::from("");
}

fn print_left() -> () {
    println!("{}", get_static_prompt());
}

fn print_right() -> () {
    if let Ok(repo) = Repository::discover(".") {

        println!("[{} {} {}]", get_statuses(&repo), get_branch_name(&repo), get_remote_diff(&repo));
    } else {
        println!("[]")
    }
}

fn main() {
    match env::args().nth(1).unwrap().as_ref() {
        "--right" => print_right(),
        "--left" => print_left(),
        &_ => (),
    }
}
