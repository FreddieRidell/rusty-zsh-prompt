use std::collections::HashMap;
use std::fmt::{self, Display};
use git2::{BranchType, Repository};

fn get_stashes(repo: &mut Repository) -> String {
    let mut buff = String::from("");

    repo.stash_foreach(|_, stash, _| {
        buff.push_str(stash);
        buff.push_str(" ");

        true
    });


    paint_text_in_color(&1, buff)
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
    fn format(&self, num: usize) -> String {
        let color = match self {
            &OutputStatuses::Conflicted => &6,
            &OutputStatuses::Deleted => &1,
            &OutputStatuses::Modified => &3,
            &OutputStatuses::New => &2,
            &OutputStatuses::Renamed => &5,
            &OutputStatuses::TypeChange => &4,
        };

        paint_text_in_color(color, format!("{}", num))
    }
}

#[derive(Debug)]
struct StatusBlock {
    statuses: HashMap<OutputStatuses, usize>,
}

impl StatusBlock {
    fn new() -> Self {
        StatusBlock {
            statuses: HashMap::new()
        }
    }

    fn increment(&mut self, status: OutputStatuses) -> () {
        self.statuses.entry(status)
            .and_modify(|e| { *e += 1 })
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
        ].iter().for_each( |status| {
            if let Some(count) = self.statuses.get(status) {
                output.push(status.format(*count));
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
    fn new () -> Self {
        Statuses {
            index: StatusBlock::new(),
            working: StatusBlock::new(),
        }
    }

    fn increment_index (&mut self, status: OutputStatuses) -> () {
        self.index.increment(status);
    }

    fn increment_working (&mut self, status: OutputStatuses) -> () {
        self.working.increment(status);
    }
}

impl Display for Statuses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.working, self.index)
    }
}

fn get_statuses(repo: &Repository) -> String {
    let mut statuses = Statuses::new();

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

    format!(" {}", statuses)
}

fn paint_text_in_color(color: &i8, text: String) -> String {
    format!("%F{{{}}}{}%f", color, text)
}

fn get_branch_name(repo: &Repository) -> String {
    if let Ok(head) = repo.head() {
        if let Some(head_name) = head.shorthand() {
            return paint_text_in_color(&5, String::from(head_name));
        }
    }

    String::from("NO_BRANCH")
}

fn get_remote_diff(repo: &Repository) -> String {
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
                        return paint_text_in_color(&4, format!("{}/{} ", ahead, behind));
                    }
                }
            }
        }
    }

    return String::from("");
}

fn print_right() -> () {
    if let Ok(mut repo) = Repository::discover(".") {
        println!(
            "[ {}{} {} {}]",
            get_stashes(&mut repo),
            get_statuses(&repo),
            get_branch_name(&repo),
            get_remote_diff(&repo)
        );
    } else {
        println!("[]")
    }
}

fn main() {
    print_right();
}
