use git2::{BranchType, Repository};
use std::collections::HashMap;
use std::fmt::{self, Display};

fn get_stashes(use_ansi: bool, repo: &mut Repository) -> String {
    let mut buff = String::from("");

    let result = repo.stash_foreach(|_, stash, _| {
        buff.push_str(stash);
        buff.push_str(" ");

        true
    });

    if let Err(_) = result {
        panic!()
    };

    paint_text_in_color(use_ansi, &1, buff)
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum OutputStatuses {
    Conflicted,
    Deleted,
    Modified,
    New,
    Renamed,
    TypeChange,
}

impl OutputStatuses {
    fn format(&self, use_ansi: bool, num: usize) -> String {
        let color = match self {
            &OutputStatuses::Conflicted => &6,
            &OutputStatuses::Deleted => &1,
            &OutputStatuses::Modified => &3,
            &OutputStatuses::New => &2,
            &OutputStatuses::Renamed => &5,
            &OutputStatuses::TypeChange => &4,
        };

        paint_text_in_color(use_ansi, color, format!("{}", num))
    }
}

#[derive(Debug)]
struct StatusBlock {
    use_ansi: bool,
    statuses: HashMap<OutputStatuses, usize>,
}

impl StatusBlock {
    fn new(use_ansi: bool) -> Self {
        StatusBlock {
            use_ansi,
            statuses: HashMap::new(),
        }
    }

    fn increment(&mut self, status: OutputStatuses) -> () {
        self.statuses
            .entry(status)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
}

impl Display for StatusBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();

        vec![
            OutputStatuses::Conflicted,
            OutputStatuses::Deleted,
            OutputStatuses::Modified,
            OutputStatuses::New,
            OutputStatuses::Renamed,
            OutputStatuses::TypeChange,
        ]
        .iter()
        .for_each(|status| {
            if let Some(count) = self.statuses.get(status) {
                output.push(status.format(self.use_ansi, *count));
            }
        });

        write!(f, "{}", output.join(" "))
    }
}

#[derive(Debug)]
struct Statuses {
    index: StatusBlock,
    working: StatusBlock,
}

impl Statuses {
    fn new(use_ansi: bool) -> Self {
        Statuses {
            index: StatusBlock::new(use_ansi),
            working: StatusBlock::new(use_ansi),
        }
    }

    fn increment_index(&mut self, status: OutputStatuses) -> () {
        self.index.increment(status);
    }

    fn increment_working(&mut self, status: OutputStatuses) -> () {
        self.working.increment(status);
    }
}

impl Display for Statuses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.working, self.index)
    }
}

fn get_statuses(use_ansi: bool, repo: &Repository) -> String {
    let mut statuses = Statuses::new(use_ansi);

    repo.statuses(None)
        .unwrap()
        .iter()
        .for_each(|file_statuses| -> () {
            if file_statuses.status().is_index_new() {
                statuses.increment_index(OutputStatuses::New);
            };
            if file_statuses.status().is_index_modified() {
                statuses.increment_index(OutputStatuses::Modified);
            };
            if file_statuses.status().is_index_deleted() {
                statuses.increment_index(OutputStatuses::Deleted);
            };
            if file_statuses.status().is_index_renamed() {
                statuses.increment_index(OutputStatuses::Renamed);
            };
            if file_statuses.status().is_index_typechange() {
                statuses.increment_index(OutputStatuses::TypeChange);
            };
            if file_statuses.status().is_wt_new() {
                statuses.increment_working(OutputStatuses::New);
            };
            if file_statuses.status().is_wt_modified() {
                statuses.increment_working(OutputStatuses::Modified);
            };
            if file_statuses.status().is_wt_deleted() {
                statuses.increment_working(OutputStatuses::Deleted);
            };
            if file_statuses.status().is_wt_typechange() {
                statuses.increment_working(OutputStatuses::TypeChange);
            };
            if file_statuses.status().is_wt_renamed() {
                statuses.increment_working(OutputStatuses::Renamed);
            };
            if file_statuses.status().is_conflicted() {
                statuses.increment_working(OutputStatuses::Conflicted);
            };
        });

    format!("{}", statuses)
}

fn paint_text_in_color(use_ansi: bool, color: &i8, text: String) -> String {
    if use_ansi {
        format!("\x1b[{}m{}\x1b[0m", (color + 30).to_string(), text)
    } else {
        format!("%F{{{}}}{}%f", color, text)
    }
}

fn get_branch_name(use_ansi: bool, repo: &Repository) -> String {
    if let Ok(head) = repo.head() {
        if let Some(head_name) = head.shorthand() {
            return paint_text_in_color(use_ansi, &5, String::from(head_name));
        }
    }

    String::from("NO_BRANCH")
}

fn get_remote_diff(use_ansi: bool, repo: &Repository) -> String {
    if let Ok(head) = repo.head() {
        if let Some(shorthand) = head.shorthand() {
            let branch_name = String::from(shorthand);

            if let Ok(this_branch) = repo.find_branch(branch_name.as_str(), BranchType::Local) {
                let this_oid = repo
                    .refname_to_id(this_branch.get().name().unwrap())
                    .unwrap();
                if let Ok(upstream_branch) = this_branch.upstream() {
                    let upstream_oid = repo
                        .refname_to_id(upstream_branch.get().name().unwrap())
                        .unwrap();

                    if let Ok((ahead, behind)) = repo.graph_ahead_behind(this_oid, upstream_oid) {
                        return paint_text_in_color(use_ansi, &4, format!("{}/{} ", ahead, behind));
                    }
                }
            }
        }
    }

    return String::from("");
}

fn print_right(use_ansi: bool) -> () {
    if let Ok(mut repo) = Repository::discover(".") {
        println!(
            "[ {}{} {} {}]",
            get_stashes(use_ansi, &mut repo),
            get_statuses(use_ansi, &repo),
            get_branch_name(use_ansi, &repo),
            get_remote_diff(use_ansi, &repo)
        );
    } else {
        println!("[]")
    }
}
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    ansi: bool,
}

fn main() {
    let Args { ansi } = Args::parse();

    print_right(ansi);
}
