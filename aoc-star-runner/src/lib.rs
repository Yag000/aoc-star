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

