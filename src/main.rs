use git2::Repository;
use git2::Status;
use std::env;

use std::collections::HashMap;

fn paint_text_in_color(color: &i8, text: String) -> String {
    format!("%F{{{}}}{}%f", color, text)
}

fn paint_text_with_bg(color: &i8, text: String) -> String {
    format!("%K{{{}}}{}%k", color, text)
}

fn get_static_prompt() -> String {
    vec![
        paint_text_in_color(&1, String::from("%n@%m")),
        paint_text_in_color(&7, String::from("%D{%a %Y-%m-%d %T}")),
        paint_text_in_color(&2, String::from("%~")),
        paint_text_in_color(&7, String::from("\n$ ")),
    ]
    .join(" ")
}

fn get_repo() -> String {
    String::from("%{%F{white}%}(%{%F{magenta}%}repo%{%f%})")
}

fn get_newline() -> String {
    String::from("\n%f$ ")
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
            match (statuses_count.get(status)) {
                Some(n) => paint_text_in_color(color, format!("{}", n)),
                None => String::from(""),
            }
        })
    .filter( |s| s.len() > 0)
        .collect::<Vec<String>>()
        .join(" ");

    let working_string = git_statuses_working
        .iter()
        .map(|(status, color)| -> String {
            match (statuses_count.get(status)) {
                Some(n) => paint_text_in_color(color, format!("{}", n)),
                None => String::from(""),
            }
        })
    .filter( |s| s.len() > 0)
        .collect::<Vec<String>>()
        .join(" ");

    format!(" {}|{}", working_string, staged_string)
}

fn print_left() -> () {
    println!("{}", get_static_prompt());
}
fn print_right() -> () {
    if let Ok(repo) = Repository::discover("."){
        println!("[{} {} ]", get_statuses(&repo), get_branch_name(&repo));
    } else {
        println!("[]")
    }
}

fn main() {

    match (env::args().nth(1).unwrap().as_ref()) {
        "--right" => print_right(),
        "--left" => print_left(),
        &_ => (),
    }
}
