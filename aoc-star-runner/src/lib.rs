use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub struct CommandArgument {
    #[clap(short, long)]
    pub day: Option<u32>,

    #[clap(short, long)]
    pub year: Option<i32>,

    pub part: Option<u32>,

    #[clap(short, long)]
    pub publish: bool,

    #[clap(long)]
    pub all: bool,
}

pub struct AocEntry {
    pub day: u32,
    pub part: u32,
    pub year: Option<i32>,
    pub func: fn(CommandArgument),
}

inventory::collect!(AocEntry);

pub fn run() {
    let args = CommandArgument::parse();

    if args.all {
        for entry in inventory::iter::<AocEntry> {
            println!("Running day {} part {}", entry.day, entry.part);
            (entry.func)(args.clone());
        }
        return;
    }

    let Some(day) = args.day else {
        eprintln!("--day is required unless --all is used");
        return;
    };

    let Some(part) = args.part else {
        eprintln!("part argument is required unless --all is used");
        return;
    };

    let entry = inventory::iter::<AocEntry>()
        .find(|e| e.day == day && e.part == part && args.year.is_none_or(|y| e.year == Some(y)));

    match entry {
        Some(e) => (e.func)(args),
        None => eprintln!("No solution found for day {} part {}", day, part),
    }
}
